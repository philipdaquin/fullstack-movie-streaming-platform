use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use crate::graphql::config::{create_schema, configure_service};
use crate::db::{establish_connection, session, DBCONN};
// use crate::graphql::modules::resolver::batch_indexing_into_es;
use crate::kafka::{create_producer};
use crate::telemetry::init_telemetry;
use tracing_actix_web::TracingLogger;
use std::fs::File;
use std::io::Write;

pub async fn new_server(port: u32) -> std::io::Result<()> {
    init_telemetry();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // If TRUE: it means you are creating a NEW database
    // if FALSE: it means you are connecting to an EXISTING scylla cluster
    let db_pool = establish_connection()
        .await
        .expect("Unable to establish ScyllaDB connection");
    
    // Initialise Kafka Producer for batch indexng
    let kafka_producer = create_producer();
    log::info!("Welcome to Apache Kafka 🦿Batch Indexer ");

    // ** MANUAL IMPLEMENTATION OF BATCH INDEXING 
    //  If true, the service will get all the items in the database 
    // and insert them in a queue to be Reindexded by elasticsearch
    // batch_indexing_into_es(*REINDEX_TO_ELASTIC_SEARCH, db_pool)
    //     .await
    //     .expect("Error Received from Batch Indexing Kafka");
    //  Automate writing new subgraphs
    let schema = web::Data::new(create_schema(db_pool));
    let app_name = format!("{}.graphql", env!("CARGO_PKG_NAME"));
    let mut subgraph = File::create(app_name.clone())
        .expect(format!("Unable to create a subgraph file for {}", app_name.clone().as_str()).as_str())
        .write_all(schema.sdl().as_bytes())
        .expect(format!("Unable to write new GraphQL Type into {}", app_name.as_str()).as_str());

    log::info!("{}", &schema.sdl());
    log::info!("🚀 Starting HTTP server on port {} ", port);
    log::info!("📭 GraphiQL playground: http://localhost:{}/graphiql", port);
    log::info!("📢 Query at https://studio.apollographql.com/dev");
    log::info!("⛽⛽ Running Jaeger: 
    You can launch one using Docker:
        `docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest`
        
        You can look at the exported traces in your browser by visiting http://localhost:16686.
        Spans will be also printed to the console in JSON format, as structured log records.
    ");
    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();

    HttpServer::new(move || {
        App::new()
            .app_data(kafka_producer.clone())
            .app_data(schema.clone())
            .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // .wrap(TracingLogger::default())
    })
    .workers(3)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
