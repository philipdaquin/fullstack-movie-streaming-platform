
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder, HttpResponse, HttpRequest, guard, Result};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema, Context, extensions::ApolloTracing,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::db::{DbPool};
use super::root_schema::{Mutation, Query, AppSchema};
use redis::{
    aio::ConnectionManager as RedisManager, 
    Client as RedisClient, 
};


pub fn configure_service(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(graphql)
    .service(graphql_playground)
    .service(
        web::resource("/graphiql")
            .route(web::get()
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws)
        )
    );
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
/// GraphiQL playground UI
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

pub async fn index_ws(
    schema: web::Data<AppSchema>, 
    req: HttpRequest, 
    payload: web::Payload) -> Result<HttpResponse, actix_web::Error> { 
    GraphQLSubscription::new(Schema::clone(&*schema))
        .start(&req, payload)
}

pub fn create_schema(
    pool: DbPool,
    redis_pool: RedisClient, 
    redis_connection: RedisManager
) -> AppSchema { 
    // Caching Service 
    let arc_redis_connection = Arc::new(redis_connection);

    Schema::build(Query::default(), Mutation::default(), EmptySubscription
    )
    .enable_federation()
    .data(arc_redis_connection)
    // Add a global data that can be accessed in the Schema
    //  Redis Caching Client  
    .data(redis_pool)
    //  SQL Database Pool
    // Add a global data that can be accessed in the Schema
    .data(pool)
    .extension(ApolloTracing)
    .finish()
}
// Run migrations. TODO: only do this on dev environment
pub async fn run_migrations(pool: &DbPool) { 
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to run database migrations");
}

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> DbPool { 
    let pool = ctx.data::<DbPool>()
        .expect("Failed to get Db Pool");
    pool.clone()
}

/// Access Redis from the Context, use 'create_connection' to establish connection asynchronously
pub async fn get_redis_conn_from_ctx(ctx: &Context<'_>) -> RedisClient { 
    ctx.data::<RedisClient>()
        .expect("Failed to get Redis Client")
        .clone()
}
/// Access Redis Database Connection
pub async fn get_redis_conn_manager(ctx: &Context<'_>) -> RedisManager { 
    ctx.data::<RedisManager>()
        .expect("Failed to get Redis Connection Manager")
        .clone()
}