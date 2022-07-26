use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use crate::db::intiliase_index;
use crate::{db::create_client};
use crate::kafka_dualwrites::{KAFKA_BROKER, create_consumer_dual_writes, run_consumer_group_dual_writes};
use tracing_actix_web::TracingLogger;
use crate::telemetry::init_telemetry;
use std::fs::File;
use std::io::Write;

pub async fn new_server(port: u32) -> std::io::Result<()> {
    init_telemetry();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Establish Elastic Search client connection 
    let elastic_search = create_client()
        .await
        .expect("Unable to get elasticsearch client");
    // let schema = web::Data::new(create_schema(elastic_search.clone()));
    let _ = intiliase_index(elastic_search)
        .await
        .expect("Unable to create Index");

    log::info!("🚀 Starting HTTP server on port {} ", port);
    log::info!("🍻 Welcome to Elasticsearch and ELK, Elastic is accessible through Kabana: 'http://localhost:5601/app/home#/'", );
    log::info!("🎢 Welcome to Apache Kafka {} ", KAFKA_BROKER.as_str());
    
    let _ = create_consumer_dual_writes().expect("Unable to create a consumer for Kafka");
    let _ = run_consumer_group_dual_writes().await;

    opentelemetry::global::shutdown_tracer_provider();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    
    .workers(3)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

/// Helper function to automate setting new databases
pub fn batch_index() -> bool { 
    dotenv::dotenv().ok();
    let user_input = std::env::var("BATCH_INDEX").expect("Unable to read Batch Index");
    if user_input.as_str() == "true" { 
        return true;
    } else { 
        return false;
    }
}
pub fn recreate_index() -> bool { 
    dotenv::dotenv().ok();
    let user_input = std::env::var("RECREATE_INDEX").expect("Unable to read RECREATE_INDEX");
    if user_input.as_str() == "true" { 
        return true;
    } else { 
        return false;
    }
}
