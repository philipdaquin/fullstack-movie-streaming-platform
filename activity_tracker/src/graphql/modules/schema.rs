use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::graphql::modules::model::KafkaType;
use crate::{graphql::modules::model::UserWatchTime, kafka};
use crate::graphql::config::get_conn_from_ctx;
use serde_json::Value;
use super::resolver::{AnalyticsResolver, AnalyticsDatabase};

#[derive(Default)]
pub struct AnalyticsQuery; 
pub struct UserType { 
    pub id: ID
}
#[Object(extends)]
impl UserType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID { &self.id}
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserAnalytics { 
    pub time: i64,
    pub movie_id: i64,
    pub session: i64,
    pub title: String,
    pub user_id: String, 
    pub liked: bool
}

#[Object]
impl AnalyticsQuery { 
    #[graphql(entity)]
    async fn get_user(&self, #[graphql(key)] id: ID) -> UserType { 
        UserType { id }
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getAllRecords")]
    async fn get_all_records(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserAnalytics>> { 
        let res = UserWatchTime::get_all_records::<AnalyticsDatabase>(get_conn_from_ctx(ctx))
            .await
            .expect("")
            .into_iter()
            .map(|f| UserAnalytics::from(f)).collect();
        Ok(res) 
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getUserRecords")]
    async fn get_user_records(&self, ctx: &Context<'_>, user_id: i64) -> FieldResult<Vec<UserAnalytics>> { 
        let res = UserWatchTime::get_user_records::<AnalyticsDatabase>(user_id, get_conn_from_ctx(ctx))
            .await
            .expect("")
            .into_iter()
            .map(|f| UserAnalytics::from(f)).collect();
        Ok(res) 
    } 
}

#[derive(Default)]
pub struct AnalyticsMutation;

#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct UserInfoInput { 
    pub movie_id: i64,
    pub user_id: i64, 
    pub session: i64,
    pub title: String, 
    pub liked: Option<bool>
}
#[Object]
impl AnalyticsMutation { 
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "recordUser")]
    async fn record_user(&self, ctx: &Context<'_>, user_info: UserInfoInput) -> Vec<UserAnalytics> { 
        let res: Vec<UserAnalytics> = UserWatchTime::record_user_watchtime::<AnalyticsDatabase>(
            UserWatchTime::from(&user_info),
            get_conn_from_ctx(ctx)
        ).await
        .expect("Unable to get the User Analytics")
        .into_iter()
        .map(|f| UserAnalytics::from(f))
        .collect();
        log::info!("Received from the database! {:#?}", res);

        //  Test 
        let kafka_type: Vec<KafkaType> = res.clone()
            .into_iter()
            .map(|f| KafkaType::from(f))
            .collect();
            
        //  Publish new message to kafka so it can be delivered to 
        //  tensorflow to be analysed along with the movie datasets
        let message = serde_json::to_string(&kafka_type).expect("Unable to serialize movie");
        kafka::send_message(&message).await;
        res
    }
}
