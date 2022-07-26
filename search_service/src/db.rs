use std::time::Duration;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, BulkUpdateOperation, BulkOperation, Reindex, SearchParts};
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
use once_cell::sync::OnceCell;
use url::Url;
use elasticsearch::cert::CertificateValidation;
use serde_json::{json, Value};


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

pub static INDEX_NAME: OnceCell<String> = OnceCell::new();
#[inline]
pub(crate) fn index_name() -> String {
    INDEX_NAME.get().expect("Unable to get Elasticsearch index").to_string()
}




lazy_static! { 
    static ref ELASTICSEARCH_URL: String = std::env::var("ELASTICSEARCH_URL").unwrap_or_else(|_| DEFAULT_ADDRESS.into()); 
    static ref ELASTIC_USER: String = std::env::var("ELASTIC_USER").unwrap_or_else(|_| "elastic".into());
    static ref ELASTIC_PASSWORD: String = std::env::var("ELASTIC_PASSWORD").expect("");
    static ref PROXY_URL: String = std::env::var("PROXY_URL").expect("Unable to validate Proxy Url");
    static ref CLOUD_ID: String = std::env::var("CLOUD_ID").expect("Unable to validate Cloud Id");
    static ref CLOUD_CLUSTER_NAME: String = std::env::var("CLOUD_CLUSTER_NAME").expect("Unable to validate Cloud Cluster");
    static ref CLOUD_CLUSTER_URL: String = std::env::var("CLOUD_CLUSTER_URL").expect("Unable to validate Cloud Url");
}

/// Establishes connection to an ElasticSearch Cluster
/// Builds a transport to make API requests to ElasticSearch using the TransportBuilder ,
/// which allows setting of proxies, authentication schemes, certification validation, and other transport related settings
#[tracing::instrument(level = "debug", err)]
pub async fn establish_connection() -> Result<Elasticsearch, BuildError> { 
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
    let _ = INDEX_NAME.set(std::env::var("INDEX_NAME").expect("Invalid Index Name"));

    Ok(elastic_search)
}
/// Using the cat indices API tag the following information for each index in a cluster: 
/// - Shard count
/// - Document count
/// - Deleted document count
/// - Primary store size
/// - Total store size of all shards, including shard replicas
pub async fn cat_indices() -> Result<Response, Error> {
    let response = elastisearch_client()
        .0
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .send()
        .await;

    response
}

/// Example search query
/// Could be useful if this was implemented with GraphQl and Apollo, though I think 
/// Elastic already provides a neat way of searching through an index using something called 
/// AppSearch Node
pub async fn search_api(
    client: Elasticsearch,
    query: serde_json::Value,
    query_number_of_results: Option<i64>,
    index: &String,
) -> Value {
    let number_of_results: i64 = match query_number_of_results {
        Some(n) => n,
        None => 10,
    };

    let response: Value = client
        .search(SearchParts::Index(&[index]))
        .from(0)
        .size(number_of_results)
        .body(query)
        .pretty(true)
        .send()
        .await
        .expect("Unable to Send")
        .error_for_status_code()
        .expect("Invalid status code")
        .json()
        .await
        .expect("Unable to Deserialize");

    response
}

