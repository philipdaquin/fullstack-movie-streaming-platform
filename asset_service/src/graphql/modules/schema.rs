use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDate;
use rdkafka::producer::FutureProducer;
use strum_macros::{Display, EnumString};
use super::{model::{Movie, Status, BusinessData, MovieRating}, resolver::MovieDatabase};
use crate::{graphql::{config::get_conn_from_ctx}, to_bigint, to_int, kafka};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCompanyType { 
    pub company_id: ID
}
#[Object(extends)]
impl ProductionCompanyType { 
    #[graphql(external)]
    pub async fn company_id(&self) -> &ID { &self.company_id }
}

#[derive(Default)]
pub struct MovieQuery;
#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct MovieType { 
    pub movie_id: ID,
    pub title: String,
    pub year: i32,
    pub awards: Vec<String>,
    pub business: BusinessData,
    pub countries: Vec<String>,
    pub genres: Vec<String>,
    pub homepage: String,
    pub keywords: Vec<String>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub movie_casts: Vec<String>,
    pub movie_company: Vec<String>,
    pub movie_director: Vec<String>,
    pub movie_writer: Vec<String>,
    pub overview: String,
    pub poster: String,
    pub rated: String,
    pub rating: MovieRating,
    pub release_date: NaiveDate,
    pub runtime: i64,
    pub status: String,
    pub video_file: String,
}

#[Object(extends, cache_control(max_age = 180))]
impl MovieQuery  {

    #[graphql(entity)]
    async fn get_company_by_id(&self, #[graphql(key)] company_id: ID) -> ProductionCompanyType { 
        ProductionCompanyType { company_id }
    }

    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getAllMovies")]
    async fn get_all(&self, ctx: &Context<'_>, page_size: Option<i32>) -> FieldResult<Vec<MovieType>> { 
        let res = Movie::get_all_movie::<MovieDatabase>(get_conn_from_ctx(ctx), page_size)
            .await
            .expect("")
            .iter()
            .map(|g| MovieType::from(g))
            .collect();
        Ok(res)
    }
    #[graphql(name = "getMovieById")]
    async fn get_by_movie_id(&self, ctx: &Context<'_>, title: String, id: ID) -> FieldResult<MovieType> { 
        let movie = find_movie_internally(ctx, title, id).await.expect("");
        Ok(MovieType::from(&movie))
    }
    #[graphql(entity, name = "getMovieByIdEntitity")]
    async fn get_by_movie_entity(&self, ctx: &Context<'_>, title: String, #[graphql(key)] movie_id: ID) -> FieldResult<MovieType> { 
        let movie = find_movie_internally(ctx, title, movie_id).await.expect("");
        Ok(MovieType::from(&movie))
    }

    /// Keeping Elasticsearch in sync 
    /// Only accessible by an admin
    /// Queue items into Kafka and into ElasticSearch
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "ForcebatchIndexIntoElasticsearch")]
    async fn force_batch_indexing_into_es(&self, ctx: &Context<'_>, page_size: Option<i32>) -> FieldResult<Vec<MovieType>> { 
        let res = Movie::get_all_movie::<MovieDatabase>(get_conn_from_ctx(ctx), page_size)
            .await
            .expect("Unable to Get all the items inside the database");
        let response = res
            .iter()
            .map(|f| MovieType::from(f))
            .collect();

        //  Batching based on Ranges 
        for i in res { 
            // Publish new message to a specific topic 
            // This goes into Kafka to be sent to the Elastic Search where Movies can be indexed
            log::info!("🚢🚢 Received Client Request to Sync Data back into Elasticsearch: {:#?}", i);
            let message = serde_json::to_string(&i).expect("Unable to serialize movie");
            kafka::send_message(&message).await;
        }
        Ok(response)
    }
    /// Our search indexing platform is more reliable if the search service can call the movie to be indexed
    /// the incremental indexing pseed helps refresh data faster and appears more promptly in our consumer applications 
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "ForceIndexMovieByID")]
    async fn force_index_movie_by_id(&self, ctx: &Context<'_>, movie_id: ID, movie_name: String) -> FieldResult<MovieType> { 
        let movie = find_movie_internally(ctx, movie_name, movie_id)
            .await
            .expect("");
        log::info!("🚢🚢 Received Client Request to Sync Data back into Elasticsearch: {:#?}", movie);
        let message = serde_json::to_string(&movie).expect("Unable to serialize movie");
        kafka::send_message(&message).await;
        Ok(MovieType::from(&movie))
    }

}

async fn find_movie_internally(ctx: &Context<'_>, title: String, id: ID) -> FieldResult<Movie> {
    let res = Movie::get_movie_id_title::<MovieDatabase>(
        title, to_bigint(id), get_conn_from_ctx(ctx))
        .await
        .expect("");
    Ok(res)
} 
