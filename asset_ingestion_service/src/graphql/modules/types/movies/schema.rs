use async_graphql::*;
use async_graphql_actix_web::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::{graphql::{config::get_conn_from_ctx, modules::types::{prod_company::{schema::ProductionCompanyType, resolver::CompanyDetailsLoader}, tmdb_test::{fetch_movies_externally, fetch_movies_by_list, fetch_movie_details}}}, to_bigint, kafka};
use super::{model::{BusinessData, MovieRating, MediaType, MediaRated, Status, Movie, NewMovie}, resolver::MovieDatabase};
use async_graphql::dataloader::*;

#[derive(Default)]
pub struct MovieMutation;

#[derive(SimpleObject,  Debug, Clone, Deserialize, Serialize)]
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

// #[Object]
// impl MovieType { 
//     async fn movie_company(&self, ctx: &Context<'_>) -> FieldResult<ProductionCompanyType> { 
//         let loader = ctx
//             .data::<DataLoader<CompanyDetailsLoader>>()
//             .expect("Unable to load external dataloaders");
//         let company_details = loader
//             .load_one(self.movie_company[0].clone())
//             .await
//             .expect("");
//         // let company_details = loader
//         //     .load_many(&&mut self.movie_company[..])
//         //     .await?
//         //     .expect("Unable to get company details via Dataloader");
//         company_details.ok_or_else(|| "Not found".into())
//     }
// }


#[derive(InputObject, Debug, Clone, Deserialize, Serialize)]
pub struct NewMovieInput { 
    /// Movie Title; not null 
    #[graphql(validator(max_length = 50))]
    pub title: String,
    /// Year; defaulted to 0
    pub year: Option<i32>,
    /// A list of awards for this movie
    #[graphql(validator(list, max_length = 60))]
    pub awards: Option<Vec<String>>,
    /// BusinessDate representing the movie budget and revenue
    pub business: Option<BusinessDataInput>,
    /// A list of countries associated in the movie
    #[graphql(validator(list, max_length = 50))]
    pub countries: Option<Vec<String>>,
    /// A list of genres, provided no unique Id as Indexing through 
    /// Elasticsearch will make it easier and cheaper filter through 
    #[graphql(validator(list, max_length = 200))]
    pub genres: Option<Vec<String>>,
    /// URI link to movie's homepage
    pub homepage: Option<String>,
    /// List of keywords of indexing purposes 
    #[graphql(validator(list, max_length = 200))]
    pub keywords: Option<Vec<String>>,
    /// List of languages 
    #[graphql(validator(list, max_length = 200))]
    pub languages: Option<Vec<String>>,
    /// Type of media; defaulted to Movie
    pub media_type: Option<MediaType>,
    /// List of MovieCasts names, people who were in the movie 
    /// Each person is searchable 
    #[graphql(validator(list, max_length = 400))]
    pub movie_casts: Option<Vec<String>>,
    /// List of Production Companies that worked in making this movie 
    /// Each company is searchable 
    #[graphql(validator(list, max_length = 400))]
    pub movie_company: Option<Vec<String>>,
    /// As inferred
    // #[graphql(validator(list))]
    pub movie_director: Option<Vec<String>>,
    /// As inferred
    #[graphql(validator(list, max_length = 100))]
    pub movie_writer: Option<Vec<String>>,
    /// Movie plot, 
    #[graphql(validator(max_length = 400))]
    pub overview: Option<String>,
    /// S3-CDN image link
    pub poster: Option<String>,
    /// MediaRated: R, PG, Pg13, Nc17, G
    pub rated: Option<MediaRated>,
    /// Based on IMDB Ratings
    pub rating: Option<MovieRatingInput>,
    /// Released of Movie YYYY-MM-DD
    pub release_date: Option<NaiveDate>,
    /// In minutes
    pub runtime: Option<i64>,
    /// Current state of the movie
    pub status: Option<Status>,
    /// S3 Media Link
    pub video_file: Option<String>,
}

#[derive(Debug, Clone, InputObject, Serialize, Deserialize, Default)]
pub struct BusinessDataInput { 
    pub budget: Option<i64>, 
    pub revenue: Option<i64>
}

#[derive(Debug, Clone, InputObject, Serialize, Deserialize, Default)]
pub struct MovieRatingInput { 
    pub imdb_id: Option<String>,
    pub metascore: Option<i32>, 
    pub popularity: Option<f32>,
    pub vote_count: Option<i64>, 
    pub vote_average: Option<f32>
}
#[derive(Debug, Clone, InputObject, Serialize, Deserialize, Default)]
pub struct BulkStreamInsertData { 
    pub discover_api: String, 
    pub endpoint_popular: String, 
    pub language: Option<String>,
    pub included_with: Option<String>,
    pub number_of_batch: Option<i32>
}

#[Object]
impl MovieMutation { 
    #[tracing::instrument(skip(self, ctx), fields(new_movie))]
    #[graphql(name = "createMovie")]
    async fn create_movie(&self, ctx: &Context<'_>, new_movie: NewMovieInput) -> FieldResult<MovieType> { 
        let res = Movie::create_movie::<MovieDatabase>(NewMovie::from(&new_movie), get_conn_from_ctx(ctx))
            .await
            .expect("Unable to convert Movie to proper Graphql Type");
    
        log::info!("🚢🚢 Sending over to Kafka {:#?}", res);
        // Publish new message to a specific topic 
        // This goes into Kafka to be sent to the Elastic Search where Movies can be indexed
        let message = serde_json::to_string(&res).expect("Unable to serialize movie");
        kafka::send_message(&message).await;

        Ok(MovieType::from(&res))
    }
    /// From Elastic.co
    /// Overwriting the document in Elasticsearch is just as efficient as an update operation would be, because 
    /// internally an update would consist of deleting the old document and then indexing an entirely new document 
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "updateMovie")]
    async fn update_movie(&self, ctx: &Context<'_>, new_movie: NewMovieInput, movie_id: ID) -> FieldResult<MovieType> { 
        let res = Movie::update_movie::<MovieDatabase>(to_bigint(movie_id), NewMovie::from(&new_movie), get_conn_from_ctx(ctx))
            .await
            .expect("");
        // Publish new message to a specific topic 
        // This goes into Kafka to be sent to the Elastic Search where Movies can be indexed
        let message = serde_json::to_string(&res).expect("Unable to serialize movie");
        kafka::send_message(&message).await;

        Ok(MovieType::from(&res))
    }
    /// There are two ways of doing this:
    /// One way is to include a field 'is_deleted' to indicate that they are no longer valid
    /// The other way is, in this context, instead of sending Kafka messages to elasticsearch where ordering or queing isnt necessary
    /// We can simply call a GRAPHQL API directly that signals to ELasticsearch to delete the corresponding documents
    /// Steps: 
    /// Delete from Scylla DB
    /// Delete Document from Elasticsearch
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "deleteMovie")]
    async fn delete_movie(&self, ctx: &Context<'_>, movie_id: ID, title: String) -> FieldResult<bool> { 
        //  First delete from the Scylla Db
        let res = Movie::delete_movie::<MovieDatabase>(to_bigint(movie_id), title, get_conn_from_ctx(ctx))
            .await
            .expect("Unable to delete the specified rows");
        Ok(res)
    }
    /// Bulk inserting dataset from TMDB, USED FOR database query analysis and optimisation
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "batchInsertData")]
    async fn batch_insert_test_data(&self, ctx: &Context<'_>, insert_data: BulkStreamInsertData) -> FieldResult<Vec<MovieType>> {
        let BulkStreamInsertData {
            discover_api, 
            endpoint_popular, 
            language, 
            included_with, 
            number_of_batch
        } = insert_data;
        log::info!("🚢 Running Bulk data insert");

        let movie_list = fetch_movies_by_list(
            discover_api, 
            endpoint_popular.clone(), 
            language.clone(), 
            included_with, 
            number_of_batch
        )
            .await
            .expect("Unable to get a list of movies"); 
        let movie_details = fetch_movie_details(
            movie_list.clone(),
            endpoint_popular, 
            language
        )
            .await
            .expect("Unable to get each details");
        
        let res = Movie::bulk_insert::<MovieDatabase>(
            movie_details.clone(), 
            get_conn_from_ctx(ctx)
        )
            .await
            .expect("Unable to execute batch query");

        let mut stream = futures::stream::iter(movie_details.clone());
        while let Some(movie) = stream.next().await { 
            // Publish new message to a specific topic 
            log::info!("📦 Loading {:#?}", movie.clone());
            // This goes into Kafka to be sent to the Elastic Search where Movies can be indexed
            log::info!("🚢 Received Client Request to Sync Data back into Elasticsearch: {:#?}", res);
            let message = serde_json::to_string(&movie).expect("Unable to serialize movie");
            kafka::send_message(&message).await;
        }
        Ok(
            movie_details
            .iter()
            .map(|g| MovieType::from(g))
            .collect()
        )
    }
    /// Bulk inserting dataset from TMDB, USED FOR database query analysis and optimisation
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "streamInsertData")]
    async fn stream_insert_test_data(&self, ctx: &Context<'_>, insert_data: BulkStreamInsertData) -> FieldResult<Vec<MovieType>> {
        let BulkStreamInsertData {
            discover_api, 
            endpoint_popular, 
            language, 
            included_with, 
            number_of_batch
        } = insert_data;
        log::info!("🐃 Running Stream data insertion");
        let movie_list = fetch_movies_by_list(
            discover_api, 
            endpoint_popular.clone(), 
            language.clone(), 
            included_with, 
            number_of_batch
        )
            .await
            .expect("Unable to get a list of movies"); 
        let movie_details = fetch_movie_details(
            movie_list.clone(),
            endpoint_popular, 
            language
        )
            .await
            .expect("Unable to get each details");
        
        let res = Movie::stream_insert::<MovieDatabase>(
            movie_details.clone(), 
            get_conn_from_ctx(ctx)
        )
            .await
            .expect("Unable to execute batch query");
        
        let mut stream = futures::stream::iter(movie_details.clone());
        while let Some(movie) = stream.next().await { 
            // Publish new message to a specific topic 
            log::info!("📦 Loading {:#?}", movie.clone());
            // This goes into Kafka to be sent to the Elastic Search where Movies can be indexed
            log::info!("🚢 Received Client Request to Sync Data back into Elasticsearch: {:#?}", res);
            let message = serde_json::to_string(&movie).expect("Unable to serialize movie");
            kafka::send_message(&message).await;
        }
        Ok(
            movie_details
            .iter()
            .map(|g| MovieType::from(g))
            .collect()
        )
    }
    
}