use async_trait::async_trait;
use chrono::naive::NaiveDate;
use common_utils::QueryResult;
use scylla::IntoTypedRows;
use crate::{db::CachedSession, kafka};
use super::model::{Movie};
use futures::stream::StreamExt;



#[async_trait]
pub trait MovieResolver { 
    async fn get_all_movie(session: &'static CachedSession, page_size: Option<i32>) -> QueryResult<Vec<Movie>>;
    async fn get_movie_by_id_title(title: String, movie_id: i64, session: &'static CachedSession) -> QueryResult<Movie>;

}

pub struct MovieDatabase;

static GET_ALL_MOVIES: &str = "select * from movie_keyspace.movies_object;";
static GET_MOVIE_BY_ID_AND_TITLE: &str = "SELECT * FROM movie_keyspace.movies_object WHERE title = ? AND movie_id = ? ;";


#[async_trait]
impl MovieResolver for MovieDatabase {  
    /// 'Get All Movies' Runs a prepared query with paging. This method will query all pages of the result
    /// 
    /// Returns an async iterator (stream) over all received rows
    /// Page size can be specified in the PRepared Statement passed to the function
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"))]
    async fn get_all_movie(session: &'static CachedSession, page_size: Option<i32>) -> QueryResult<Vec<Movie>> {
        // let mut temp = vec![];
        // let mut rows = session
        //     .query_iter(GET_ALL_MOVIES, (), page_size)
        //     .await
        //     .expect("")
        //     .into_typed::<Movie>();
        // while let Some(next_item) = rows.next().await { 
        //     log::info!("{:#?}", next_item.clone().unwrap());
        //     temp.push(next_item.unwrap())
        // }
        let rows = session
            .query_prepared(GET_ALL_MOVIES, ())
            .await
            .expect("")
            .rows_or_empty()
            .into_typed::<Movie>()
            .map(|v| v.expect(""))
            .collect();

        Ok(rows)
    }            
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movies_object"))]
    async fn get_movie_by_id_title(title: String, movie_id: i64, session: &'static CachedSession) -> QueryResult<Movie>  {
        let res = session
            .query_prepared(GET_MOVIE_BY_ID_AND_TITLE, (title, movie_id))
            .await
            .expect("")
            .rows
            .unwrap_or_default()
            .into_typed::<Movie>()
            .next()
            .unwrap()
            .unwrap();
        Ok(res)
    }
}

