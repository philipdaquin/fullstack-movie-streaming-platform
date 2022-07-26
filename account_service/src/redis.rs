use std::sync::Mutex;
use actix_web::{web::Data, HttpResponse};
use redis::aio::ConnectionManager;
use redis::{Client, RedisError, ToRedisArgs, RedisResult, aio::Connection};
use common_utils::error::ServiceError;
use std::env;
use lazy_static::lazy_static;


use crate::graphql::{
    profile_module::schema::ProfileType,
    user_module::schema::UserType
};

pub const NEW_POST_USER_CACHE: &str = "newBlogPostofUser";


lazy_static! { 
    static ref BLOG_KEY_PREFIX: String = std::env::var("REDIS_KEY_PREFIX").expect("JWT Secret Key Error");
}

pub enum RedisDatabase { 
    Example, 
    ExampleSet
}
/// Connect to Redis 
pub async fn create_client(cache: RedisDatabase) -> Result<Client, RedisError> {
    let redis_url = match cache { 
        RedisDatabase::Example => env::var("REDIS_URL").expect("Cannot Read Redis URI"),
        _ => env::var("REDIS_URL_TEST").expect("Cannot Redis TEST URL")
    };

    Ok(Client::open(redis_url)?)
}
/// Establish connection to redis 
pub async fn create_connection(redis_client: Client) -> Result<Connection, ServiceError> { 
    redis_client
        .get_async_connection()
        .await
        .map_err(|_| ServiceError::ServerError("Unable to create Redis Connection".into()))
} 
/// Post Caching Key 
pub fn get_post_cache_key(id: &str) -> String { 
    format!("{}:{}", BLOG_KEY_PREFIX.as_str(), id)
}

/// Write the Value into a vector of bytes, in this case, we are caching a Post
/// and turning into a string so we can use it as an argument for K-V pair
/// 'write_redis_args', each item is a single argument
impl ToRedisArgs for UserType { 
    fn write_redis_args<W>(&self, out: &mut W)
    where
            W: ?Sized + redis::RedisWrite {
        out.write_arg_fmt(serde_json::to_string(self)
            .expect("Cannot Serialize PostObject as String")
        )   
    }
}

impl ToRedisArgs for ProfileType { 
    fn write_redis_args<W>(&self, out: &mut W)
    where
            W: ?Sized + redis::RedisWrite {
        out.write_arg_fmt(serde_json::to_string(self)
            .expect("Cannot Serialize PostObject as String")
        )   
    }
}

//  Redis Pub/ Sub
//  Senders are not programmed to send their messages to specific receivers 
//  Rather, they will publish messages irrespectively without having the
//  knowledge of what subscribers there may be 
pub async fn start_pubsub(client: &Client) -> Result<(), ServiceError> { 
    // let mut pubsub_conn = create_connection(client).await?.into_pubsub();

    todo!()
}