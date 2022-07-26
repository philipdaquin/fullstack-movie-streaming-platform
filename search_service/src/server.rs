use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::{db::establish_connection, graphql::config::{create_schema, configure_service}};
use std::fs::File;
use std::io::Write;

/// Creates a new server instanc e
pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_pool = establish_connection()
        .await
        .expect("Unable to establish Elaticsearch client connection");
    let schema = web::Data::new(create_schema(db_pool));
    
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
    log::info!("👋 Welcome to Elasticsearch, the best way to interact with Elasticsearch is through Kabana: http://localhost:5601");
    
    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(4)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}