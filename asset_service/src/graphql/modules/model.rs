use async_graphql::dataloader::Loader;
use scylla::macros::FromRow;
use async_graphql::{Enum, SimpleObject};
use chrono::naive::{NaiveDate};
use common_utils::{QueryResult, default_date};
use scylla::{ValueList};
use scylla::macros::{FromUserType, IntoUserType};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};
use chrono::Utc;
use crate::db::CachedSession;
use crate::{generate_unique_id, to_int};
use super::resolver::MovieResolver;
use super::schema::MovieType;
use scylla::cql_to_rust::FromCqlVal;

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, Clone, SimpleObject, IntoUserType, FromUserType, ValueList, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct BusinessData { 
    pub budget: i64, 
    pub revenue: i64
}

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, Clone, SimpleObject, IntoUserType, FromUserType, ValueList, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct MovieRating { 
    pub imdb_id: String,
    pub metascore: i32, 
    pub popularity: f32,
    pub vote_count: i64, 
    pub vote_average: f32
}

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, ValueList)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Movie { 
    pub movie_id: i64,
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

#[derive(Copy, Clone, Eq, Debug, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status { 
    Rumoured,
    Planned,
    InProduction,
    PostProduction,
    Released,
    Canceled,
}

impl From<&Movie> for MovieType { 
    fn from(f: &Movie) -> Self {
        let movie_id = f.movie_id.into();
        log::info!("Deserializing..  \n{:#?}", f);

        Self { 
            movie_id, 
            title: f.title.clone() ,
            year: f.year.clone() ,
            awards: f.awards.clone() ,
            business: f.business.clone() ,
            countries: f.countries.clone() ,
            genres: f.genres.clone(),
            homepage: f.homepage.clone() ,
            keywords: f.keywords.clone() ,
            languages: f.languages.clone(),
            media_type: f.media_type.clone() ,
            movie_casts: f.movie_casts.clone(),
            movie_company: f.movie_company.clone(),
            movie_director: f.movie_director.clone(),
            movie_writer: f.movie_writer.clone(),
            overview: f.overview.clone() ,
            poster: f.poster.clone() ,
            rated: f.rated.clone() ,
            rating: f.rating.clone() ,
            release_date: f.release_date.clone() ,
            runtime: f.runtime.clone() ,
            status: f.status.clone() ,
            video_file: f.video_file.clone() ,
            
        }
    }
}


impl Movie { 
    #[tracing::instrument(skip(session))]
    pub async fn get_all_movie<MovieDatabase: MovieResolver>(session: &'static CachedSession, page_size: Option<i32>) -> QueryResult<Vec<Movie>> {
        MovieDatabase::get_all_movie(session, page_size).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn get_movie_id_title<MovieDatabase: MovieResolver>(title: String, movie_id: i64, session: &'static CachedSession) -> QueryResult<Movie> {
        MovieDatabase::get_movie_by_id_title(title, movie_id, session).await
    }
}