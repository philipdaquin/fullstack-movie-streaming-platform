use std::fmt::Display;
use serde_tuple::*;
use chrono::{Duration, Local, NaiveDateTime};
use common_utils::QueryResult;
use influx_db_client::{Series, Point, Value};
use serde_json::{Value as JsonValue, Number};
use serde::{Serialize, Deserialize};
use crate::db::InfluxDBClient;
use thiserror::Error;
use super::{resolver::{AnalyticsDatabase, AnalyticsResolver}, schema::{UserInfoInput, UserAnalytics}};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserWatchTime { 
    pub time: i64,
    pub liked: bool,
    pub movie_id: i64,
    pub session: i64,
    pub title: String, 
    pub user_id: i64, 
}
#[derive(Debug, Clone, PartialEq)]
pub struct NewUserWatchTime { 
    pub time: i64,
    pub liked: bool,
    pub movie_id: i64,
    pub session: i64,
    pub title: String, 
    pub user_id: i64, 
}

impl From<&UserInfoInput> for UserWatchTime { 
    fn from(f: &UserInfoInput) -> Self {
        let time = Local::now().timestamp();
        Self {      
            time,
            user_id: f.user_id.clone(),
            movie_id: f.movie_id.clone(),
            session: f.session.clone(),
            title: f.title.clone(),
            liked: f.liked.clone().unwrap_or_default()
        }
    }
} 

impl From<UserWatchTime> for UserAnalytics { 
    fn from(f: UserWatchTime) -> Self {
        Self {
            time: f.time.clone(),
            movie_id: f.movie_id.clone(),
            session: f.session.clone(),
            title: f.title.clone(),
            user_id: f.user_id.clone().to_string(),
            liked: f.liked.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KafkaType { 
    pub title: String, 
    pub user_id: String,
}

impl From<UserAnalytics> for KafkaType { 
    fn from(f: UserAnalytics) -> Self {
        Self {
            title: f.title.clone(),
            user_id: f.user_id.clone(),
        }
    }
}


impl From<Vec<JsonValue>> for UserWatchTime {
    fn from(f: Vec<JsonValue>) -> Self {

        log::info!("🏭 Deserializing .. {:#?}", f);
        Self {
            time: f[0].as_i64().unwrap_or_default(),
            liked: f[1].as_str().unwrap_or_default().parse::<bool>().unwrap_or_default(),
            movie_id: f[2].as_str().unwrap_or_default().parse::<i64>().unwrap_or_default(),
            session: f[3].as_str().unwrap_or_default().parse::<i64>().unwrap_or_default(),
            title: f[4].as_str().unwrap_or_default().to_string(),
            user_id: f[5].as_i64().unwrap_or_default(),
        }
    }
}

impl UserWatchTime { 
    pub fn insert_new_record(user_info: UserWatchTime) -> Point {
        let point = Point::new(&std::env::var("INLFUXDB_DBNAME").unwrap())
            .add_tag("session", Value::Integer(user_info.session as i64))
            .add_field("user_id", Value::Integer(user_info.user_id))
            .add_tag("movie_id", Value::Integer(user_info.movie_id as i64))
            .add_tag("title", Value::String(user_info.title.to_string()))
            .add_tag("liked", Value::Boolean(user_info.liked as bool))
            .add_timestamp(Local::now().timestamp())
            .clone();
            log::info!("Pint {:#?}", point);
        point
    }
}

impl UserWatchTime { 
    pub async fn record_user_watchtime<Record: AnalyticsResolver>(user_info: UserWatchTime, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        Record::record_user_watchtime(user_info, client).await
    }
    pub async fn get_all_records<Record: AnalyticsResolver>(client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        Record::get_all_records(client).await
    }
    pub async fn get_user_records<Record: AnalyticsResolver>(user_id: i64, client: InfluxDBClient) -> QueryResult<Vec<UserWatchTime>> { 
        Record::get_user_records(user_id, client).await
    }
}