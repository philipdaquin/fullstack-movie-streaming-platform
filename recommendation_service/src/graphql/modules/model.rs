use common_utils::QueryResult;
use scylla::{FromRow, ValueList};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

use crate::db::CachedSession;

use super::{resolver::RecommendedTrait, schema::RecommendedType};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ValueList)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct RecommendedMovies { 
    pub user_id: i32,
    pub movie_id: i64,
    pub created_at: NaiveDate,
    pub title: String,
}

impl From<&RecommendedMovies> for RecommendedType { 
    fn from(f: &RecommendedMovies) -> Self {
        Self {
            user_id: f.user_id.clone(),
            movie_id: f.movie_id.clone(),
            created_at: f.created_at.clone(),
            title: f.title.clone(),
        }
    }
}

impl RecommendedMovies { 
    pub async fn get_most_recent<Rn: RecommendedTrait>(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> {
        Rn::get_most_recent(user_id, session).await
    }
    pub async fn get_all_recommendations<Rn: RecommendedTrait>(session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> {
        Rn::get_all_recommendations(session).await
    }
    pub async fn get_user_recommendations<Rn: RecommendedTrait>(user_id: i32, session: &'static CachedSession) -> QueryResult<Vec<RecommendedMovies>> {
        Rn::get_user_recommendations(user_id, session).await
    }

}




























































