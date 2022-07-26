
use std::{sync::Arc, borrow::Borrow};

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder, HttpResponse, HttpRequest, guard};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema, Context, extensions::ApolloTracing, dataloader::DataLoader,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::db::{CachedSession, session};
use super::{root_schema::{Mutation, Query, AppSchema, AppSchemaBuilder}, 
    modules::types::prod_company::resolver::CompanyDetailsLoader
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

pub fn create_schema(pool: &'static CachedSession) -> AppSchema { 
    let dataloader = DataLoader::new(
        CompanyDetailsLoader {pool}, 
        tokio::spawn
    ).max_batch_size(10);
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    // Add a global data that can be accessed in the Schema
    .data(dataloader)
    .data(pool)
    .extension(ApolloTracing)
    .finish()
}
/// Database Pool Connection 
pub fn get_conn_from_ctx(ctx: &Context<'_>) -> &'static CachedSession { 
    let pool = ctx.data::<&'static CachedSession>()
        .expect("Failed to Connect to Database");
    pool.clone()
}

