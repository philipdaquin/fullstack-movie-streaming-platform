use async_graphql_actix_web::*;
use async_graphql::*;
use chrono::NaiveDate;
use common_utils::QueryResult;
use serde::{Deserialize, Serialize};
use crate::db::index_name;
use crate::graphql::config::get_conn_from_ctx;

use super::{model::{Movie, BusinessData, MovieRating, FilterQueryWithMultipleFields, Genre, SimpleSearchNew}, resolver::{ElasticResolver, ElasticDatabase}};




#[derive(Default)]
pub struct ElasticQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCompanyType { 
    pub company_id: ID
}
#[Object(extends)]
impl ProductionCompanyType { 
    #[graphql(external)]
    pub async fn company_id(&self) -> &ID { &self.company_id }
}

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


#[derive(InputObject, Clone, Debug)]
pub struct FilterQuery { 
    pub term_name: Option<String>,
    pub term_value: Option<String>,
    pub total_result: Option<i64>,
    pub index_name: String
}
#[derive(InputObject, Clone, Debug)]
pub struct SortAllMovies { 
    pub term_name: Option<String>,
    pub order: Option<String>,
    pub total_result: Option<i64>,
    pub index_name: String
}

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct AggregatedQuery { 
    pub genres: Option<Vec<Genre>>,
    pub movie_list: Option<Vec<MovieType>>
}

#[derive(Debug, Clone, InputObject, Deserialize, Serialize)]
pub struct SearchTextInput { 
    /// Default Value to be Null or an empty string
    pub query: Option<String>, 
    /// Default Value is set to 2 search results
    pub total_result: Option<i64>,

    #[graphql(default = "index_name()")]
    pub index_name: String,
    /// Default Value of Sorting is made for 
    /// Ratings.Popularity, ASC
    pub sort_by: Option<String>,
    /// Defaulted to ASC
    pub order: Option<String>,
    /// Defaulted Agg field to Genres.Keyword
    pub agg_field: Option<String>, 
    /// Defaulted size to 2
    pub agg_size: Option<i32>,
    ///  Filter Search result by:
    /// Default Value set to genres
    pub filter_by: Option<String>,
    /// Value to filter the searches
    /// Default set to empty Strimng
    pub filter_value: Option<String>

}


#[Object]
impl ElasticQuery { 
    #[graphql(entity)]
    async fn get_company_by_id(&self, #[graphql(key)] company_id: ID) -> ProductionCompanyType { 
        ProductionCompanyType { company_id }
    }
    /// Retrieve all indexed movies
    #[graphql(name = "searchAll")]
    async fn search_all(
        &self, 
        ctx: &Context<'_>, 
        total_result: Option<i64>, 
        #[graphql(default = "index_name()")]
        index_name: String
    ) -> Vec<MovieType> { 
        let res = Movie::search_indexed::<ElasticDatabase>(get_conn_from_ctx(ctx), total_result, index_name)
            .await
            .expect("Unable to get the movie using genre_id")
            .iter()
            .map(|f| MovieType::from(f))
            .collect();
        res
    }
    /// Retrieve all indexed movies
    #[graphql(name = "searchMovie")]
    async fn search_phrase_prefix(&self, ctx: &Context<'_>, input: SearchTextInput) -> Option<AggregatedQuery> { 
        let res = Movie::search_phrase_prefix::<ElasticDatabase>(
            get_conn_from_ctx(ctx), 
            SimpleSearchNew::from(&input))
            .await
            .expect("Unable to get any result for both");
        log::info!("📦 Genre List {:#?}, MovieList {:#?}", res.genres, res.movie_list);
        Some(res)
    }

    /// Execute this query under index_name with 
    /// Filter by term_name: term_value and give total_results
    #[graphql(name = "filterBy")]
    async fn filter_by(
        &self, 
        ctx: &Context<'_>,
        filter: FilterQuery
    ) -> Vec<MovieType> { 
        let res = Movie::filter_by::<ElasticDatabase>(
            filter.term_name.unwrap_or_default(), 
            filter.term_value.unwrap_or_default(), 
            get_conn_from_ctx(ctx),
            filter.total_result,
            filter.index_name
        ) 
            .await
            .expect("Unable to retrieve the items")
            .iter()
            .map(|f| MovieType::from(f))
            .collect();
        res
    }
    /// For the sake of simplicity and ease of use in the frontend,
    /// and after facing several alongside, I decided to get rid of using here struct
    #[graphql(name = "searchWithAggregatedFilter")]
    async fn search_with_filter(
        &self, 
        ctx: &Context<'_>,
        query: Option<String>,
        term_name: Option<String>,
        term_value: Option<String>, 
        total_result: Option<i64>,
        #[graphql(default = "index_name()")]
        index_name: String,
        fields: Option<Vec<String>>,
        sort_by: Option<String>,
        order: Option<String>
    ) -> Vec<MovieType> { 
        let res = Movie::filter_or_aggregate_query::<ElasticDatabase>(
            FilterQueryWithMultipleFields::new(
                query, 
                term_name, 
                term_value, 
                total_result, 
                index_name,
                fields,
                sort_by,
                order
            ),get_conn_from_ctx(ctx))
            .await
            .expect("Unable to retrieve the items")
            .iter()
            .map(|movie| MovieType::from(movie))
            .collect();
        res
    }
    /// Default Values of sort is Descending
    #[graphql(name = "sortMoviesAccordingly")]
    async fn sort_movie_based(&self, ctx: &Context<'_>, input: SortAllMovies) -> Vec<MovieType> { 
        let res = Movie::sort_movies_by::<ElasticDatabase>( 
            input.term_name,
            input.order,
            get_conn_from_ctx(ctx),
            input.total_result,
            input.index_name
        )
            .await
            .expect("")
            .iter()
            .map(|f| MovieType::from(f))
            .collect();
        res
    }


}

#[derive(Default)]
pub struct ElasticMutate;

#[Object]
impl ElasticMutate { 
    /// Deletes the index under this id 
    #[graphql(name = "deleteMovieDocByID")]
    async fn delete_document_by_id(&self, ctx: &Context<'_>, movie_id: ID) -> QueryResult<bool> { 
        let res = Movie::delete_document::<ElasticDatabase>(movie_id.as_str(), get_conn_from_ctx(ctx))
            .await
            .expect("");
        Ok(res)
    } 
}