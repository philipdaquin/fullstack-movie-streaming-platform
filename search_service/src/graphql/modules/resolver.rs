use elasticsearch::{Elasticsearch, DeleteParts};
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::graphql::modules::model::Movie;
use crate::db::{search_api, INDEX_NAME, index_name};
use crate::graphql::modules::schema::MovieType;
use common_utils::QueryResult;
use super::model::{FilterQueryWithMultipleFields, Genre, SimpleSearchNew};
use super::schema::AggregatedQuery;

/*
    ALL THE CODE BELOW ESPECIALLY ELASTICSEARCH QUERIES ARE ALL 
    SUBJECT TO CHANGE, MOST OF THEM ARE CONSTRUCTED TO BE BUILT ON TOP OF 
    EACH OTHER 🥸

    The error checks is performed on the frontend, or in other words, the front end 
    client decides which api to use, hence the 
*/

/// field name: ratings.popularity, order: asc
pub fn sort_by(field_name: String, order: String) -> Value { 
    json!(
        {
        field_name: {
            "order": order
          }
        }
    )
}

/// ONLY SEARCH 
pub fn match_all() -> Value { 
    json!({ 
        "query": {
            "match_all": {}
        } 

    })
}
pub fn match_phrase_prefix(query: String) -> Value { 
    json!({ 
        "query": {
            "match_phrase_prefix": {
              "title": query
            }
        }
    })
}
/// Only filter
/// Example: genre : "Action"
pub fn only_filter_val(term: String, term_value: String) -> Value { 
    json!({
        "query": {
            "bool": {
              "filter": [
                {
                  "term": { term: term_value }
                }
              ]
            }
        }
    })
} 
/// Example: ratings.popularity
/// order: desc
/// Get all the movies and sort them based on popularity 
pub fn sort_all_movies(term_name: String, order: String) -> Value { 
    json!({
        "query": {
            "match_all": {}
        }, 
        "sort": [
          {
            term_name: {
              "order": order
            }
          }
        ]
    })
}

pub fn multi_match(
    query: String, 
    agg_field: String, 
    agg_size: i32, 
    sort_by: String, 
    order: String,
    filter_by: String,
    filter_value: String
) -> Value { 
    json!({
        "query": {
            "multi_match": {
              "query": query,
                "fields": [
                    "title^5", 
                    "overview", 
                    "keywords",
                    "keywords.keyword", 
                    "countries", 
                    "media_type.keyword",
                    "movie_casts",
                    "genres",
                    "movie_company",
                    "movie_writer",
                    "status.keyword"
                ]
            }
        },
        "aggs": {
            "genres_collect": 
            {
                "terms": {
                "field": agg_field,
                "size": agg_size
                }
            },
            "filter_by_genre": {
                "filter": {
                  "term": 
                  {
                    filter_by.clone(): filter_value
                  }
                }
            }
        },
        "sort": [
            {
                sort_by: {
                    "order": order
                }
            }
        ],
        "post_filter": {
            "term": {
                filter_by: filter_value
            }
        }
    })
}


/// COMBINATIONS
/// The default values for each of the inputs are found in the models 
/// file. Essentially, sort_by and order is defaulted to ratings.popularity and asc 
/// and agg_field and agg_size is defaulted to ""  and 1. This ensures that an unfounded 
/// input always gives us non error or at least make data available as much as possible
pub fn search_movie_val(
    query: String, 
    sort_by: String, 
    order: String,
    agg_field: String,
    agg_size: i32,
    filter_by: String,
    filter_value: String
) -> Value { 
    if query.len() == 0 { 
        // automatically sort based on popularity/ most trending movie 
       return aggregate_filter_by(
            agg_field, 
            agg_size,
            sort_by,
            order
        )
    } else if query.len() > 0 { 
        //  Include the ability to query non-title fields 
        log::info!("Running match phrase prefix");
         return json!({ 
            "query": {
                "match_phrase_prefix": {
                  "title": query
                }
            },
            "sort": [{
                sort_by: {
                "order": order
              }
            }]
            ,
            "aggs": {
                "genres": {
                    "terms": 
                    {
                        "field": agg_field,
                        "size": agg_size
                    }
                },
                "filter_by_genre": {
                    "filter": {
                      "term": 
                      {
                        filter_by.clone() : filter_value
                      }
                    }
                }
            }
            // ,
            // "post_filter": {
            //     "term": {
            //         filter_by: filter_value
            //     }
            // }
        })
        
    //  ** Check the console for log errors, 
    // } else if query.len() > 3 { 
    //     log::trace!("👏 Running a Match Phrase"); 
    //     return multi_match(
    //         query, 
    //         agg_field, 
    //         agg_size, 
    //         sort_by,
    //         order,
    //         filter_by,
    //         filter_value
    //     )
      
    } else if query.len() > 4 {
        log::trace!("👏 Running a Multimatch"); 
        return  json!({
            "query": {
                "match_phrase": {
                    "overview": {
                      "query": query
                    },
                    "slop": 4
                }
            },
            "aggs": {
                "genres": {
                "terms": 
                    {
                    "field": agg_field,
                    "size": agg_size
                    }
              },
                "filter_by_genre": {
                    "filter": {
                      "term": 
                      {
                        filter_by.clone(): filter_value
                      }
                    }
                }
            },
            "sort": [{
                sort_by: {
                "order": order
              }
            }]
            // Post Filter, Removed all of the remaining search results
            // "post_filter": {
            //     "term": {
            //         filter_by: filter_value
            //     }
            //   }
        })    
    } else { 
        return search_filter_aggregate(
            query, 
            filter_by, 
            filter_value, 
            sort_by,
            order)
    }
}
///  SEARCH WITH FILTERS ON 
pub fn filter_search_query(query: String, term_name: String, term_value: String, fields: Vec<String>,sort_by: String, order: String ) -> Value { 
    return json!({
        "query": {
            "bool": {
              "must": [
                {
                  "term": {
                    term_name: {
                      "value": term_value
                    }
                  }
                }
              ],
              "should": [
                {
                  "query_string": {
                    "fields": fields,
                    "query": query
                  }
                }
              ]
            }
        },
        "sort": [
            {
                sort_by: {
                    "order": order
                }
            }
        ]
    })
}

/// The function will let me aggregate all the genres in our database and 
/// then use that as option to filter our results further, look at below  example
pub fn aggregate_filter_by(
    agg_field: String, 
    agg_size: i32,
    sort_by: String,
    order: String
) -> Value { 
    json!({
        "query": {
            "match_all": {}
        },
          "aggs": {
              "genres": {
              "terms": {
                "field": agg_field,
                "size": agg_size
              }
          }
        },
        "sort": [{
              sort_by: {
                "order": order
              }
            }
        ]
        
    })
}

/// This query will let us do three things: query by text, filter results and sort 
/// This is utilised when the user realises he/she can filter it down based on genres 
/// 
pub fn search_filter_aggregate(
    query: String, 
    filter_by: String, 
    filter_val: String, 
    sort_by: String,
    order: String
) -> Value { 
    json!({
        "query": {
            "multi_match": {
            "query": query,
            "fields": [
              "title^5", 
              "overview", 
              "keywords",
              "keywords.keyword", 
              "countries", 
              "media_type.keyword",
              "movie_casts",
              "genres",
              "movie_company",
              "movie_writer",
              "status.keyword"
      
              ]
            }
          },
          "aggs": {
            "genre": {
              "filter": {
                "term": {
                   filter_by: filter_val
                }
              }
            }
          },
          "sort": [
            {
                sort_by: {
                    "order": order
                }
            }
          ]
    })
}

// /// Needed when the uses doesnt search anything
// pub fn no_search_only_filter(
//     filter_by: String,
//     filter_value: String,
//     sort_by: String, 
//     order: String
// ) -> Value { 
//     json!({
//         "query": {
//             "bool": {
//               "filter": [
//                 {"term": {
//                     filter_by: filter_value
//                 }}
//               ]
//             }
//           },
//           "aggs": {
//             "genress": {
//                 "filter": {
//                   "term": {
//                     filter_by: filter_value
//                   }
//                 }
//               }
//           },
//           "sort": [
//             {
//                 sort_by: {
//                   "order": order
//               }
//             }
//         ]
//     })
// }



#[async_trait]
pub trait ElasticResolver { 
    async fn search_indexed(
        client: Elasticsearch, 
        total_result: Option<i64>, 
        index_name: String
    ) -> QueryResult<Vec<Movie>>;
    async fn search_phrase_prefix(
        client: Elasticsearch, 
        query: SimpleSearchNew
    ) -> Option<AggregatedQuery>;
    async fn delete_document(id: &str, client: Elasticsearch) -> QueryResult<bool>;
    async fn filter_by(
        term: String, 
        term_value: String, 
        client: Elasticsearch,
        total_result: Option<i64>, 
        index_name: String
    ) -> QueryResult<Vec<Movie>>;
    async fn filter_or_aggregate_query(
        query: FilterQueryWithMultipleFields,
        client: Elasticsearch,
    ) -> QueryResult<Vec<Movie>>; 
    async fn sort_movies_by(
        term_name: Option<String>, 
        order: Option<String>, 
        client: Elasticsearch,
        total_result: Option<i64>, 
        index_name: String 
    ) -> QueryResult<Vec<Movie>>;

}
pub struct ElasticDatabase;

#[async_trait]
impl ElasticResolver for ElasticDatabase { 
    #[tracing::instrument(skip(client), err, level = "debug")]
    async fn search_indexed(client: Elasticsearch, total_result: Option<i64>, index_name: String) -> QueryResult<Vec<Movie>> { 
        let payload = search_api(
            client,
            match_all(), 
            total_result, 
            &index_name
        ).await;

        let movies: Vec<Movie> = payload["hits"]["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|res| serde_json::from_value(res["_source"].clone()).unwrap())
            .collect();

        Ok(movies)
    }
    async fn search_phrase_prefix(
        client: Elasticsearch, 
        query: SimpleSearchNew
    ) -> Option<AggregatedQuery> { 
        let SimpleSearchNew{ 
            query, 
            total_result, 
            index_name,
            sort_by,
            order,
            agg_field,
            agg_size,
            filter_by,
            filter_value
        } = query;
        log::info!("👏 Received Order!");
        let payload = search_api(
            client,
            search_movie_val(
                query, 
                sort_by, 
                order, 
                agg_field,
                agg_size,
                filter_by,
                filter_value
            ), 
            Some(total_result), 
            &index_name
        ).await;
        log::info!("Loading the Payload 🚅🚅 {:#?}", payload);

        let genres = payload["aggregations"]["genres"]["buckets"]
            .as_array()
            .unwrap_or(&vec![Value::Null])
            .iter()
            .map(|count| serde_json::from_value(count.clone()).unwrap())
            .collect();
        log::info!("Packaging all the {:#?}", genres);
        let movie_list: Option<Vec<Movie>> = payload["hits"]["hits"]
            .as_array()
            .unwrap_or(&vec![Value::Null])
            .into_iter()
            .map(|res| serde_json::from_value(res["_source"].clone()).unwrap())
            .collect();
        log::info!("💻 Retrieving all the movie list {:#?}", movie_list);


        let movie_list: Option<Vec<MovieType>> = if movie_list.is_some() {
            let res = movie_list
                .unwrap()
                .iter()
                .map(|f| MovieType::from(f))
                .collect::<Vec<MovieType>>();
            Some(res)
        } else {
            None
        };
        log::info!("Processing the orders, Send it over back to the user 🛫");

        Some(AggregatedQuery { 
            genres,
            movie_list
        } )
    }
    // Delete item in our Elasticsearch cluster 
    #[tracing::instrument(level = "debug", err)]
    async fn delete_document(movie_id: &str, client: Elasticsearch) -> QueryResult<bool> {  
        log::info!("Deleting movie document {} from index Movies", movie_id);
        let index_name = index_name();
        
        let delete_part = DeleteParts::IndexId(&index_name, movie_id);
        let response = client
            .delete(delete_part)
            .send()
            .await
            .expect("Unable to perform deletion");
        log::info!("{:#?}", response);
        Ok(true)
    }
    /// Filters items directly
    /// "term": { "media_type.keyword": "MOVIE" }
    #[tracing::instrument(level = "debug", err)]
    async fn filter_by(
        term: String, 
        term_value: String, 
        client: Elasticsearch,
        total_result: Option<i64>, 
        index_name: String
    ) -> QueryResult<Vec<Movie>> { 
        let payload = search_api(
            client, 
            only_filter_val(term, term_value),
            total_result, 
            &index_name
        ).await;

        let movies = payload["hits"]["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|res| serde_json::from_value(res["_source"].clone()).unwrap())
            .collect();
        Ok(movies)
    }
    /// Allows the user to query an item and filter the results based on 
    /// desired terms. The function is designed to handle every possible query
    /// For example, Query: "Toronto"
    /// Filter Results: Genre: Action
    #[tracing::instrument(level = "debug", err)]
    async fn filter_or_aggregate_query(
        query: FilterQueryWithMultipleFields,
        client: Elasticsearch
    ) -> QueryResult<Vec<Movie>> { 
        log::info!("👀 Entering Filter or Aggregated Query API");
        let FilterQueryWithMultipleFields { 
            query, 
            term_name, 
            term_value, 
            total_result, 
            index_name ,
            fields,
            sort_by, 
            order,
        } = query;
        
        let payload = search_api(
            client, 
            filter_search_query(
                query, 
                term_name, 
                term_value, 
                fields,
                sort_by, 
                order
            ),
            Some(total_result), 
            &index_name
        ).await;


        let movies = payload["hits"]["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| serde_json::from_value(f["_source"].clone()).unwrap())
            .collect();
        log::info!("📫 Sending over the results: {:#?}", movies);
        Ok(movies)
    }
    async fn sort_movies_by(
        term_name: Option<String>, 
        order: Option<String>, 
        client: Elasticsearch,
        total_result: Option<i64>, 
        index_name: String 
    ) -> QueryResult<Vec<Movie>> { 
        log::info!("👀 Entering Filter or Aggregated Query API");
        let payload = search_api(
            client, 
            sort_all_movies(term_name.unwrap_or_default(), order.unwrap_or("desc".to_string())),
            total_result, 
            &index_name
        ).await;
        let movies = payload["hits"]["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| serde_json::from_value(f["_source"].clone()).unwrap())
            .collect();
        log::info!("📫 Sending over the results: {:#?}", movies);
        Ok(movies)        
    }
    
}