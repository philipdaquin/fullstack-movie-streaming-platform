pub mod config;
pub mod root_schema;
pub mod utils;
pub mod profile_module;
pub mod user_module;
/// Helper Functions
use async_graphql::*;
use common_utils::error::ServiceError;
use uuid::Uuid;

pub fn to_uuid(id: ID) -> Result<Uuid, ServiceError> { 
    Uuid::parse_str(id.as_str())
        .map_err(|e| ServiceError::DatabaseError)
}