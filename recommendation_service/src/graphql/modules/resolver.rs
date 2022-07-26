use async_trait::async_trait;
use common_utils::QueryResult;
use scylla::IntoTypedRows;
use crate::db::CachedSession;
use super::model::RecommendedMovies;
#[async_trait]
pub trait RecommendedTrait { 
    async fn get_most_recent(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>>;
    async fn get_all_recommendations(session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>>;
    async fn get_user_recommendations(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>>;
}
pub struct RecommendedDatabase;

static GET_MOST_RECENT: &str = "SELECT * FROM recommended_movies.user_recommendations WHERE user_id = ? ORDER BY time DESC LIMIT 50";
static GET_ALL_RECOMMENDATIONS: &str = "SELECT * FROM recommended_movies.user_recommendations LIMIT 100";
static GET_USER_RECOMMENDATIONS: &str = "SELECT * FROM recommended_movies.user_recommendations WHERE user_id = ?";

#[async_trait]
impl RecommendedTrait for RecommendedDatabase { 
    async fn get_most_recent(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> { 
        let res = session 
            .query_prepared(GET_MOST_RECENT, (user_id,))
            .await
            .expect("Unable to get the recent User's Recommended Content")
            .rows_or_empty()
            .into_typed::<RecommendedMovies>()
            .map(|f| f.expect(""))
            .collect();
        Ok(res)
    }
    async fn get_all_recommendations(session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> {
        let res = session 
            .query_prepared(GET_ALL_RECOMMENDATIONS, ())
            .await
            .expect("Unable to get the recent User's Recommended Content")
            .rows_or_empty()
            .into_typed::<RecommendedMovies>()
            .map(|f| f.expect(""))
            .collect();
        Ok(res)
    }
    async fn get_user_recommendations(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> {
        let res = session 
            .query_prepared(GET_USER_RECOMMENDATIONS, (user_id,))
            .await
            .expect("Unable to get the recent User's Recommended Content")
            .rows_or_empty()
            .into_typed::<RecommendedMovies>()
            .map(|f| f.expect(""))
            .collect();
        Ok(res)
    }
}