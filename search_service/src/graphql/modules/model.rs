use async_graphql::SimpleObject;
use chrono::NaiveDate;
use common_utils::QueryResult;
use elasticsearch::Elasticsearch;
use serde::{Serialize, Deserialize};
use super::{resolver::ElasticResolver, schema::{MovieType, SearchTextInput, AggregatedQuery}};

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Genre { 
    pub doc_count: i32,
    pub key: String,
}



// WARNING: REPEATING CODE ALERT, but don't worry this is just trying-to-get-something-work
// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MovieRating { 
    pub imdb_id: String,
    pub metascore: i32, 
    pub popularity: f32,
    pub vote_count: i64, 
    pub vote_average: f32
}
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct BusinessData { 
    pub budget: i64, 
    pub revenue: i64
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct NewMovie { 
    pub id: i64,
    pub title: String,
    pub adult: bool,
    pub backdrop_path: String,
    pub budget: i64,
    pub first_air_date: NaiveDate,
    pub genre_ids: i32,
    pub homepage: String,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    pub production_company: i32, 
    pub production_countries: i32, 
    pub release_date: NaiveDate,
    pub runtime: i64,
    pub spoken_languages: i32,
    pub status: String,
    pub vote_average: f32,
    pub vote_count: i64,
}

impl From<&Movie> for MovieType { 
    fn from(f: &Movie) -> Self {
        let movie_id = f.movie_id.clone().into();
        log::info!("✅✅✅ \n{:#?}", f);
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


#[derive(Debug, Clone)]
pub struct FilterQueryWithMultipleFields { 
    pub query: String,
    pub term_name: String,
    pub term_value: String, 
    pub total_result: i64,
    pub index_name: String,
    pub fields: Vec<String>,
    pub sort_by: String, 
    pub order: String
}

impl FilterQueryWithMultipleFields { 
    pub fn new(
        query: Option<String>,
        term_name: Option<String>,
        term_value: Option<String>, 
        total_result: Option<i64>,
        index_name: String,
        fields: Option<Vec<String>>,
        sort_by: Option<String>,
        order: Option<String>
    ) -> Self { 
        Self { 
            query: query.unwrap_or_default(),
            term_name: term_name.unwrap_or_default(),
            term_value: term_value.unwrap_or_default(),
            total_result: total_result.unwrap_or(30),
            index_name,
            fields: fields.unwrap_or(vec![String::new()]),
            sort_by: sort_by.unwrap_or("rating.popularity".to_string()),
            order: order.unwrap_or("asc".to_string())
        }
    }
}



#[derive(Clone, Debug)]
pub struct SimpleSearchNew { 
    pub query: String, 
    pub total_result: i64,
 
    pub index_name: String,
    pub sort_by: String,
    pub order: String,

    pub agg_field: String, 
    pub agg_size: i32,

    pub filter_by: String,
    pub filter_value: String 

}


impl From<&SearchTextInput> for SimpleSearchNew { 
    fn from(f: &SearchTextInput) -> Self {
        Self {
            query: f.query.clone().unwrap_or_default(),
            total_result: f.total_result.clone().unwrap_or(30),
            index_name: f.index_name.clone(),
            sort_by: f.sort_by.clone().unwrap_or("rating.popularity".to_string()),
            order: f.order.clone().unwrap_or("asc".to_string()),
            agg_field: f.agg_field.clone().unwrap_or(String::new()),
            agg_size: f.agg_size.clone().unwrap_or(1),
            filter_by: f.filter_by.clone().unwrap_or("genres".to_string()),
            filter_value: f.filter_value.clone().unwrap_or(String::new())
        }
    }
}

impl Movie { 
    #[tracing::instrument(skip(client))]
    pub async fn search_indexed<Elastic: ElasticResolver>(client: Elasticsearch, total_result: Option<i64>, index_name: String
    ) -> QueryResult<Vec<Movie>> { 
        Elastic::search_indexed(client, total_result, index_name).await
    }
    #[tracing::instrument(skip(client))]
    pub async fn search_phrase_prefix<Elastic: ElasticResolver>(client: Elasticsearch, query: SimpleSearchNew ) -> Option<AggregatedQuery> { 
        Elastic::search_phrase_prefix(client, query).await
    }
    #[tracing::instrument(skip(client))]
    pub async fn delete_document<Elastic: ElasticResolver>(movie_id: &str, client: Elasticsearch) -> QueryResult<bool> { 
        Elastic::delete_document(movie_id, client).await
    }
    #[tracing::instrument(skip(client))]
    pub async fn filter_by<Elastic: ElasticResolver>(term: String, term_value: String, client: Elasticsearch, total_result: Option<i64>,  index_name: String) -> QueryResult<Vec<Movie>> {
        Elastic::filter_by(term, term_value, client, total_result, index_name).await
    }
    #[tracing::instrument(skip(client))]
    pub async fn filter_or_aggregate_query<Elastic: ElasticResolver>(query: FilterQueryWithMultipleFields, client: Elasticsearch) -> QueryResult<Vec<Movie>> {
        Elastic::filter_or_aggregate_query(query, client).await
    }  
    #[tracing::instrument(skip(client))]
    pub async fn sort_movies_by<Elastic: ElasticResolver>(
        term_name: Option<String>, 
        order: Option<String>, 
        client: Elasticsearch,
        total_result: Option<i64>, 
        index_name: String 
    ) -> QueryResult<Vec<Movie>> { 
        Elastic::sort_movies_by(term_name, order, client, total_result, index_name).await
    }

}