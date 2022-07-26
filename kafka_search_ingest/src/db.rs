use std::time::Duration;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, BulkUpdateOperation, BulkOperation, Reindex};
use elasticsearch::auth::Credentials;
use elasticsearch::http::request::JsonBody;
use elasticsearch::indices::{IndicesExistsParts, IndicesDeleteParts};
use elasticsearch::{Elasticsearch, Error, 
    IndexParts, GetParts, 
    DeleteParts, UpdateParts,
    indices::IndicesCreateParts,};
use elasticsearch::http::transport::{
    DEFAULT_ADDRESS, TransportBuilder, Transport, SingleNodeConnectionPool, BuildError};
use lazy_static::lazy_static;
use url::Url;
use elasticsearch::cert::CertificateValidation;
use once_cell::sync::OnceCell;
use serde_json::{json, Value};
use crate::module::model::{Movie, MOVIE_MAPPING};
use crate::server::recreate_index;
pub static ELASTIC_CLIENT: OnceCell<ElasticClient> = OnceCell::new();

#[inline]
pub(crate) fn elastisearch_client() -> &'static ElasticClient { 
    ELASTIC_CLIENT.get().expect("Missing Elasticsearch client")
}
pub struct ElasticClient(pub Elasticsearch);

impl From<Elasticsearch> for ElasticClient { 
    fn from(f: Elasticsearch) -> Self {
        Self(f)
    }
}
lazy_static! { 
    static ref ELASTICSEARCH_URL: String = std::env::var("ELASTICSEARCH_URL").unwrap_or_else(|_| DEFAULT_ADDRESS.into()); 
    static ref ELASTIC_USER: String = std::env::var("ELASTIC_USER").unwrap_or_else(|_| "elastic".into());
    static ref ELASTIC_PASSWORD: String = std::env::var("ELASTIC_PASSWORD").expect("");
    static ref PROXY_URL: String = std::env::var("PROXY_URL").expect("Unable to validate Proxy Url");
    static ref CLOUD_ID: String = std::env::var("CLOUD_ID").expect("Unable to validate Cloud Id");
    static ref CLOUD_CLUSTER_NAME: String = std::env::var("CLOUD_CLUSTER_NAME").expect("Unable to validate Cloud Cluster");
    static ref CLOUD_CLUSTER_URL: String = std::env::var("CLOUD_CLUSTER_URL").expect("Unable to validate Cloud Url");
    static ref INDEX_NAME: String = std::env::var("INDEX_NAME").expect("Invalid index name ");
    static ref ELASTIC_INDEX: String = std::env::var("ELASTIC_INDEX").expect("Invalid Index ");
    // static ref RECREATE_INDEX: bool = std::env::var("RECREATE_INDEX").expect("Unable to read RECREATE_INDEX").parse().unwrap();

}

/// Creates ElasticSearch Client
/// Builds a transport to make API requests to ElasticSearch using the TransportBuilder ,
/// which allows setting of proxies, authentication schemes, certification validation, and other transport related settings
#[tracing::instrument(level = "debug", err)]
pub async fn create_client() -> Result<Elasticsearch, BuildError> { 
    log::info!("🍻 Creating Elasticsearch client for {}", ELASTIC_USER.as_str());
    let url = Url::parse(ELASTICSEARCH_URL.as_ref()).expect("Unable to parse URL");    
    let credentials = Credentials::Basic(ELASTIC_USER.to_string(), ELASTIC_PASSWORD.to_string());
    let pool = SingleNodeConnectionPool::new(url);
    let builder = TransportBuilder::new(pool)
        .auth(credentials)
        .cert_validation(CertificateValidation::None)
        .enable_meta_header(true)
        .disable_proxy()
        .timeout(Duration::from_secs(60))
        .build()?;
    let elastic_search = Elasticsearch::new(builder); 
    // Store this client in OnceCell to prevent repeating code
    let _ = ELASTIC_CLIENT.set(ElasticClient::from(elastic_search.clone()));
    
    Ok(elastic_search)
}
/// Create an index API to add a new index to an Elasticsearch cluster, the index name is 
/// configurable in the ENV as 'INDEX_NAME', 'MOVIE_MAPPING' and 'MOVIE_SETTINGS'
#[tracing::instrument(skip(client), err)]
pub async fn intiliase_index(client: Elasticsearch) -> Result<bool, Error> { 
    // Run existing index
    // If a connection is successful and there are values under this index, then run accordingly 
    // else it wont hurt to create a new index when there's nothing under this index
    log::info!("💀 Initialising the index index called: {}", INDEX_NAME.as_str());
    let exist = client
        .indices()
        .exists(IndicesExistsParts::Index(&[&INDEX_NAME]))
        .error_trace(true)
        .send()
        .await?
        .status_code()
        .is_success();
    log::info!("🤔🤔 Does a connection exist under this index and are there any 
    values supporting this index? {}", exist);
    // If the user wants to recreate an old index with the same INDEX_NAME
    if recreate_index() { 
        delete_index().await.expect("");
    }
    // After several attempts to prevent "EOF while parsing a value" Error,
    // This is the simplest solution I could get, the error originated from attempting to 
    // deserialize an empty string. To solve this, It is best to return boolean rather than serde_json::Value 
    match exist { 
        true => {
            log::info!("🚅 Running an existing INDEX");
            Ok(true)
        },
        _ => {
            create_index()
                .await
                .map(|_| true)
        } 
    }
   
}
// Create a new Index
#[tracing::instrument(level = "debug", err)]
pub async fn create_index() -> Result<Response, Error> { 
    log::info!("🚨 Creating a new Index");
    let response = elastisearch_client().0.indices()
        .create(IndicesCreateParts::Index(&INDEX_NAME))
        .body(&*MOVIE_MAPPING)
        .send()
        .await?;
    Ok(response)
}
//  Delete Index 
#[tracing::instrument(level = "debug", err)]
pub async fn delete_index() -> Result<Response, Error> { 
    log::info!("⏰ Deleting Index and recreating a new version");
    let response = elastisearch_client().0.indices()
        .delete(IndicesDeleteParts::Index(&[&INDEX_NAME]))
        .send()
        .await?;
    Ok(response)
}

/// Index Movies collected from the Message Queue
/// Bulk inserting the messages from Kafka -- This is mainly done for performance reasons 
/// Assuming the volume of new movies is large, the bulk api makes it possible to perform many 
/// index operations in a singel API call. This can greatly increase the indexing speed
#[tracing::instrument(skip(movies), level = "debug", err)]
pub async fn index_movie(movies: Vec<Movie>) -> Result<Vec<Movie>, Error> { 
    log::info!("👏👏 Indexing movies from Apache Kafka: {:?}", movies);
    if movies.is_empty() { return  Ok(Vec::new()) }
    let body: Vec<BulkOperation<_>> = movies
        .iter()
        .map(|p| {
            let id = p.movie_id.to_string();
            BulkOperation::index(p).id(&id).routing(&id).into()
        })
        .collect();

    //  We can also create add a Pipeline API 
    let bulk_insert = elastisearch_client()
        .0
        .bulk(BulkParts::Index(&INDEX_NAME))
        .body(body)
        .error_trace(true)
        .send()
        .await
        .expect("Unable to perform bulk insertion");
    let response_body = bulk_insert.json::<Value>().await?;
    let err = response_body["errors"].as_bool().unwrap() == false;

    if err { 
        log::info!("🚀 Successfully imported {}", movies.len());
    } else { 
        log::info!("Failed Bulk operation: {:?}", response_body);
        // client.bulk(BulkParts::(&INDEX_NAME)
        //     .retry_on_conflict(3))
        //     .body(body)
        //     .error_trace(true)
        //     .send()
        //     .await;
    }
    log::info!("🏁🏁 Finished Indexing: {:?}", movies);

    Ok(movies)
}
/// Get APIedit
/// Retrieves the specified JSON document from an index.
#[tracing::instrument(level = "debug", err)]
pub async fn get_document(index_name: &str) -> Result<Response, Error> { 
    log::info!("🚚 Retrieving the document for: {}", index_name);
    let get_parts = GetParts::IndexId(&INDEX_NAME, index_name);
    let response = elastisearch_client()
        .0
        .get(get_parts)
        .send()
        .await
        .expect("Unable to return document");
    Ok(response)
}   
/// INDEX API
/// Adds a JSON document to the specified data stream or index and makes it searchable 
/// IF the target is an index and the document already exists, the request updates the documents
/// and increments its version
#[tracing::instrument(level = "debug", err)]
pub async fn post_document(document: &str) -> Result<Response, Error> {
    log::info!("📩 Retrieving the document for: {}", document);

    let index_parts = IndexParts::Index(&INDEX_NAME);
    let resp = elastisearch_client()
        .0
        .index(index_parts)
        .body(serde_json::from_str(document)?)
        .send()
        .await?;
    Ok(resp)    
}
/// UPDATE API
/// 
/// Requestedit
/// POST /<index>/_update/<_id>
#[tracing::instrument(level = "debug", err)]
pub async fn put_document(document: &str, id: &str) -> Result<Response, Error> {
    log::info!("🧑‍⚕️ Updating the document: {} for {}", document, id);
    let update_part = UpdateParts::IndexId(&INDEX_NAME, id);
    let response = elastisearch_client()
        .0
        .update(update_part)
        .body(serde_json::from_str(document)?)
        .send()
        .await
        .expect("Unable to return document");
    Ok(response)
}
/// Removes a JSON document from the specified index 
/// 
/// DELETE / <index>/_doc/ <_id>
#[tracing::instrument(level = "debug", err)]
pub async fn delete_document(id: &str, client: Elasticsearch) -> Result<Response, Error> {
    let delete_part = DeleteParts::IndexId(&INDEX_NAME, id);
    let response = client
        .delete(delete_part)
        .send()
        .await
        .expect("");
    Ok(response)
}
