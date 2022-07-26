use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::{graphql::config::{graphql, graphql_playground, create_schema, run_migrations, configure_service}, redis::{RedisDatabase, create_client}};
use crate::db::{DatabaseKind, establish_connection};
use std::fs::File;
use std::io::Write;

pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //  PostGreSQL Database Pool
    let db_pool = establish_connection(DatabaseKind::Example);
    run_migrations(&db_pool);
    //  Create a Redis Client `
    let redis_client = create_client(RedisDatabase::Example)
        .await
        .expect("Unable to create Redis Client Connection");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Cannot Create Redis Connection Manager");
    //  GraphQl Schema
    let schema = web::Data::new(create_schema(
        db_pool, 
        redis_client.clone(), 
        redis_connection_manager.clone()));

    //  Automate writing new subgraphs
    let app_name = format!("{}.graphql", env!("CARGO_PKG_NAME"));
    let mut subgraph = File::create(app_name.clone())
        .expect(format!("Unable to create a subgraph file for {}", app_name.clone().as_str()).as_str())
        .write_all(schema.sdl().as_bytes())
        .expect(format!("Unable to write new GraphQL Type into {}", app_name.as_str()).as_str());
    
    log::info!("{}", &schema.sdl());
    log::info!("🚀 Starting HTTP server on port {} ", port);
    log::info!("📭 GraphiQL playground: http://localhost:{}/graphiql", port);
    log::info!("📢 Query at https://studio.apollographql.com/dev");
    
    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}