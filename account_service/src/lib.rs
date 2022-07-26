extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

pub mod graphql;
pub mod server;
pub mod db;
pub mod redis;
pub mod telemetry;
use common_utils::error::{ServiceError};
use common_utils::QueryResult;


