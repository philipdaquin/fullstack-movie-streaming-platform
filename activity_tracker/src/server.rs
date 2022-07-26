use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::fs::File;
use std::io::Write;
use crate::{db::{create_client}, kafka::create_producer};
use crate::graphql::config::{graphql, graphql_playground, create_schema, configure_service};

pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let influx_client = create_client()
        .await
        .expect("Unable to get InfluxDB Client");
    let schema = web::Data::new(create_schema((*influx_client).clone()));
    
    // Initialise Kafka Producer
    let kafka_producer = create_producer();
    log::info!("Welcome to Apache Kafka 🦿");

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
    log::info!("🛫 Welcome to InfluxDB on port {}", std::env::var("INFLUXDB_URL").unwrap_or(String::from("localhost:8086")));
    log::info!("⛽⛽ Running Jaeger:  
    You can launch one using Docker: `docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest`
        You can look at the exported traces in your browser by visiting http://localhost:16686.
        Spans will be also printed to the console in JSON format, as structured log records.
    ");

    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();

    HttpServer::new(move || {
        App::new()
            .app_data(kafka_producer.clone())
            .app_data(influx_client.clone())
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

pub fn recreate_database() -> bool { 
    dotenv::dotenv().ok();
    let user_input = std::env::var("RECREATE_DATABASE").expect("Unable to read RECREATE_INDEX");
    if user_input.as_str() == "true" { 
        return true;
    } else { 
        return false;
    }
}