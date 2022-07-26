use async_graphql::*;
use chrono::NaiveDate;
use crate::graphql::config::get_conn_from_ctx;

use super::model::RecommendedMovies;
use super::resolver::{RecommendedTrait, RecommendedDatabase};

#[derive(Default)]
pub struct RecommendedQuery;
#[derive(SimpleObject, Debug, Clone)]
pub struct RecommendedType { 
    pub user_id: i32, 
    pub movie_id: i64,
    pub created_at: NaiveDate,
    pub title: String
}

pub struct UserType {
    pub id: ID
}
#[Object(extends)]
impl UserType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID { 
        &self.id
    }
    // #[graphql(name = "getUserRecommendation")]
    // pub async fn get_recommended(&self, ctx: &Context<'_>, #[graphql(key)] id: ID) -> FieldResult<Vec<RecommendedType>> { 
    //     get_user_recommended_movies(ctx, id.parse::<i32>().unwrap()).await
    // }
}

#[Object]
impl RecommendedQuery { 

    #[graphql(entity, name = "getUserByID")]
    async fn get_user(&self, #[graphql(key)] id: ID) -> UserType {
        UserType { id }
    } 

    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getUserRecentRecommendations")]
    async fn get_most_recent_movies(&self, ctx: &Context<'_>, user_id: i32) -> FieldResult<Vec<RecommendedType>> { 
        let res: Vec<RecommendedType> = RecommendedMovies::get_most_recent::<RecommendedDatabase>(user_id, get_conn_from_ctx(ctx))
            .await
            .expect("")
            .into_iter()
            .map(|f| RecommendedType::from(&f))
            .collect();
        Ok(res)
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getAllRecommendedMovies")]
    async fn get_all_recommended(&self, ctx: &Context<'_>) -> FieldResult<Vec<RecommendedType>> { 
        let res: Vec<RecommendedType> = RecommendedMovies::get_all_recommendations::<RecommendedDatabase>(get_conn_from_ctx(ctx))
            .await
            .expect("")
            .into_iter()
            .map(|f| RecommendedType::from(&f))
            .collect();
        Ok(res)
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getUserRecommendation")]
    async fn get_user_recommended(&self, ctx: &Context<'_>, user_id: i32) -> FieldResult<Vec<RecommendedType>> { 
        let res: Vec<RecommendedType> = RecommendedMovies::get_user_recommendations::<RecommendedDatabase>(user_id, get_conn_from_ctx(ctx))
            .await
            .expect("")
            .into_iter()
            .map(|f| RecommendedType::from(&f))
            .collect();
        Ok(res)
    }
    #[graphql(entity, name = "getUserRecommendations")]
    async fn get_user_recommended_entity(&self, ctx: &Context<'_>, user_id: i32) -> FieldResult<Vec<RecommendedType>> { 
        get_user_recommended_movies(ctx, user_id).await
    }
}
#[tracing::instrument(skip(ctx), level = "Debug")]
async fn get_user_recommended_movies(ctx: &Context<'_>, user_id: i32) -> FieldResult<Vec<RecommendedType>> { 
    let res: Vec<RecommendedType> = RecommendedMovies::get_user_recommendations::<RecommendedDatabase>(user_id, get_conn_from_ctx(ctx))
        .await
        .expect("")
        .into_iter()
        .map(|f| RecommendedType::from(&f))
        .collect();
    Ok(res)
}