use std::collections::HashSet;
use async_graphql::Enum;
use async_graphql::*;
use chrono::{NaiveDate, Utc, Date, Datelike};
use common_utils::QueryResult;
use scylla::macros::{FromRow, FromUserType, IntoUserType, ValueList};
use serde::{Deserialize, Serialize};
use scylla::cql_to_rust::FromCqlVal;
use strum_macros::{EnumString, Display};
use crate::{generate_unique_id, to_int};
use scylla::frame::value::{MaybeUnset, Unset};

use crate::db::CachedSession;

use super::schema::{NewMovieInput, BusinessDataInput, MovieRatingInput};
use super::{resolver::MovieResolver, schema::MovieType};
// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, Clone, SimpleObject, IntoUserType, FromUserType, ValueList, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct BusinessData { 
    pub budget: i64, 
    pub revenue: i64
}

impl BusinessData { 
    pub fn new(budget: Option<i64>, revenue: Option<i64>) -> Self { 
        Self { 
            budget: budget.unwrap_or_default(), 
            revenue: revenue.unwrap_or_default()
        }
    }
}
impl From<&BusinessDataInput> for BusinessData { 
    fn from(f: &BusinessDataInput) -> Self {
        Self {
            budget: f.budget.unwrap_or_default(),
            revenue: f.revenue.unwrap_or_default(),
        }
    }
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

impl MovieRating { 
    pub fn new(imdb_id: Option<String>, metascore: Option<i32>, popularity: Option<f32>, vote_count: Option<i64>, vote_average: Option<f32>) -> Self { 
        Self { 
            imdb_id: imdb_id.unwrap_or_default(), 
            metascore: metascore.unwrap_or_default(), 
            popularity: popularity.unwrap_or_default(), 
            vote_count: vote_count.unwrap_or_default(), 
            vote_average: vote_average.unwrap_or_default()
        }
    }
}
impl From<&MovieRatingInput> for MovieRating { 
    fn from(f: &MovieRatingInput) -> Self {
        Self {
            imdb_id: f.imdb_id.clone().unwrap_or_default(),
            metascore: f.metascore.unwrap_or_default(),
            popularity: f.popularity.unwrap_or_default(),
            vote_count: f.vote_count.unwrap_or_default(),
            vote_average: f.vote_average.unwrap_or_default(),
        }
    }
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


// Repeating Code** but with a different name, it helps identify  
// what would be an input type for generating the desired type  
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, ValueList, IntoUserType)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewMovie { 
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

#[derive(Copy, Clone, Eq, Debug, PartialEq, Serialize, SmartDefault, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status { 
    // 'Rumour' as the default value for Status
    #[default]
    Rumoured,
    Planned,
    InProduction,
    PostProduction,
    Released,
    Canceled,
}
#[derive(Copy, Clone, Eq, Debug, PartialEq, Serialize, SmartDefault, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType { 
    #[default]
    Movie,
    TvSeries,
    Trailer
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
            video_file: f.video_file.clone(),
            
        }
    }
}
// When performing an insert with values which might be NULL, it's better to use Unset.
// Database treats inserting NULL as a delete operation and will generate a tombstone. 
// Using Unset results in better performance:
impl From<&NewMovieInput> for NewMovie { 
    fn from(f: &NewMovieInput) -> Self {
        let id = generate_unique_id();
        let local: NaiveDate = Utc::today().naive_local();
        log::info!("Serializing..  \n{:#?}", f);

        let year = Utc::today().year();

        
        Self { 
            movie_id: id.clone(),
            title: f.title.clone(),
            year: f.year.unwrap_or(year.clone()),
            awards: f.awards.clone().unwrap_or(vec![String::new()]),
            business: BusinessData::from(&f.business.clone().unwrap_or_default()),
            countries: f.countries.clone().unwrap_or(vec![String::new()]),
            genres: f.genres.clone().unwrap_or(vec![String::new()]),
            homepage: f.homepage.clone().unwrap_or_default(),
            keywords: f.keywords.clone().unwrap_or(vec![String::new()]),
            languages: f.languages.clone().unwrap_or(vec![String::new()]),
            media_type: f.media_type.unwrap_or_default().to_string(),
            movie_casts: f.movie_casts.clone().unwrap_or(vec![String::new()]),
            movie_company: f.movie_company.clone().unwrap_or(vec![String::new()]),
            movie_director: f.movie_director.clone().unwrap_or(vec![String::new()]),
            movie_writer: f.movie_writer.clone().unwrap_or(vec![String::new()]),
            overview: f.overview.clone().unwrap_or_default(),
            poster: f.poster.clone().unwrap_or_default(),
            rated: f.rated.unwrap_or_default().to_string(),
            rating: MovieRating::from(&f.rating.clone().unwrap_or_default()),
            release_date: f.release_date.unwrap_or(local),
            runtime: f.runtime.unwrap_or_default(),
            status: f.status.unwrap_or_default().to_string(),
            video_file: f.video_file.clone().unwrap_or_default(),
        }
    }
}

#[derive(Copy, Clone, Eq, Debug, PartialEq, Serialize, SmartDefault, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaRated { 
    /// General audience
    G, 
    /// Parental Guidance Suggested
    #[default]    
    Pg,
    /// Parents Strongly Cautioned
    Pg_13,
    /// Restricted
    R,
    /// No one 17 and under 
    Nc_17,
}

impl Movie { 
    #[tracing::instrument(skip(session))]
    pub async fn get_movie_id<MovieDatabase: MovieResolver>(id: i64, session: &'static CachedSession) -> QueryResult<Movie> {
        MovieDatabase::get_movie_id(id, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn create_movie<MovieDatabase: MovieResolver>(new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie> {
        MovieDatabase::create_movie(new_movie, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn update_movie<MovieDatabase: MovieResolver>(id: i64, new_movie: NewMovie, session: &'static CachedSession) -> QueryResult<Movie> {
        MovieDatabase::update_movie(id, new_movie, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn delete_movie<MovieDatabase: MovieResolver>(id: i64, title: String, session: &'static CachedSession) -> QueryResult<bool> {
        MovieDatabase::delete_movie(id, title, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn bulk_insert<MovieDatabase: MovieResolver>(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool> {
        MovieDatabase::bulk_insert(movie, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn stream_insert<MovieDatabase: MovieResolver>(movie: Vec<Movie>, session: &'static CachedSession) -> QueryResult<bool> { 
        MovieDatabase::stream_insert(movie, session).await
    }


}