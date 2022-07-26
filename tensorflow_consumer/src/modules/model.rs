use scylla::ValueList;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Utc};



#[derive(Debug, Clone, Serialize, Deserialize, ValueList)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct RecommendedMovies { 
    pub user_id: i32,
    pub movie_id: i64, 
    pub created_at: NaiveDate, 
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewRecommendedMovies { 
    pub user_id: i32, 
    pub title: String,
    pub movie_id: i64,
}

impl From<NewRecommendedMovies> for RecommendedMovies { 
    fn from(f: NewRecommendedMovies) -> Self {
        let created_at =  Utc::today().naive_local();
        Self {
            user_id: f.user_id.clone(),
            title: f.title.clone(),
            created_at,
            movie_id: f.movie_id.clone()
        }
    }
} 

