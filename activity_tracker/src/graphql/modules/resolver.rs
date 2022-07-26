use async_trait::async_trait;
use common_utils::QueryResult;
use crate::{db::InfluxDBClient};
use influx_db_client::{Point, Precision, Value, Error as InfluxDbError, Series, point};
use chrono::Local;
use super::model::UserWatchTime;


#[async_trait]
pub trait AnalyticsResolver {
    async fn get_all_records(client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>>; 
    async fn get_user_records(user_id: i64, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>>;
    async fn record_user_watchtime(user_info: UserWatchTime, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>>;
}

pub struct AnalyticsDatabase;

static GET_ALL: &str = "SELECT * FROM user_activity";

#[async_trait]
impl AnalyticsResolver for AnalyticsDatabase { 
    #[tracing::instrument(skip(client), fields(repository = "user_activity"))]
    async fn get_all_records(client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        let res = client
            .query(GET_ALL, Some(Precision::Seconds))
            .await
            .expect("Unable to get all records")
            .unwrap_or_default()[0]
            .values
            .clone()
            .unwrap_or_default()
            .clone()
            .into_iter()
            .map(|f| UserWatchTime::from(f))
            .collect();
        Ok(res)
    }    
    #[tracing::instrument(skip(client), fields(repository = "user_activity"))]
    async fn get_user_records(user_id: i64, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        let query = format!("SELECT * FROM user_activity WHERE user_id = {}", user_id.to_string());
        let res = client
            .query(query.as_str(), Some(Precision::Seconds))
            .await
            .expect("Failed to get User Records")
            .unwrap_or_default()[0]
            .values
            .clone()
            .unwrap_or_default()
            .clone()
            .into_iter()
            .map(|f| UserWatchTime::from(f))
            .collect();
        log::info!("😄 Retrieving Database Values {:#?}", res);
        Ok(res)
    }
    #[tracing::instrument(skip(client), fields(repository = "user_activity"))]
    async fn record_user_watchtime(user_info: UserWatchTime, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        let point = UserWatchTime::insert_new_record(user_info.clone());
        let res = client.write(point, Some(Precision::Seconds), Some("autogen"))
            .await
            .expect("Unable to write to Influx DB");
        log::info!("{:#?}", res );
        AnalyticsDatabase::get_user_records(user_info.user_id, client).await
    }

}

