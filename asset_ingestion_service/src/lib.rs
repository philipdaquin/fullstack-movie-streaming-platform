
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

#[macro_use]
extern crate smart_default;

#[macro_use]
extern crate scylla;

#[macro_use]
extern crate derivative;

// pub mod bucket;
pub mod graphql;
pub mod server;
pub mod db;
pub mod telemetry;
pub mod kafka;
/// Global Helpers 
use async_graphql::*;
use chrono::{NaiveDate, Local};
use common_utils::error::ServiceError;
use lazy_static::lazy_static;


use common_utils::{unique_id, inc_genre};

lazy_static! { 
    static ref MACHINE_ID: i32 = std::env::var("MACHINE_ID")
        .ok()
        .and_then(|p| p.parse::<i32>().ok())
        .unwrap_or(0);
    static ref SERVER_ID: i32 = std::env::var("NODE_ID")
        .ok()
        .and_then(|p| p.parse::<i32>().ok())
        .unwrap_or(0);
}
/// Helper function to parse Async Graphql ID type into i64
pub fn to_bigint(id: ID) -> i64 { 
    id.parse::<i64>().expect("Unable to parse big int")
}
pub fn to_int(id: ID) -> i32 { 
    id.parse::<i32>().expect("Unable to parse int")

}
/// Function to generate unique identifiers based off Twitter's Snowflake Algorithm
pub fn generate_unique_id() -> i64 { 
    unique_id(*MACHINE_ID, *SERVER_ID)
}

