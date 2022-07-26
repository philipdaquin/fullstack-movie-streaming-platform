use std::sync::Mutex;
use std::time::Duration;

use futures::Future;
use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, ClientContext};
use once_cell::sync::OnceCell;

lazy_static! {
    static ref KAFKA_BROKER: String = std::env::var("KAFKA_BROKER").expect("Can't read Kafka broker address");
    static ref KAFKA_TOPIC: String = std::env::var("KAFKA_TOPIC").expect("Can't read Kafka topic name");
    static ref MESSAGE_KEY: String = std::env::var("MESSAGE_KEY").expect("Unable to get a valid message key");
    pub static ref REINDEX_TO_ELASTIC_SEARCH: bool = std::env::var("REINDEX_DATA").expect("Unable to read REINDEX_TO_ELASTIC_SEARCH").parse().unwrap();
}


pub static KAFKACONN: OnceCell<KafkaProvider> = OnceCell::new();

#[inline]
pub(crate) fn kafka_producer() -> &'static KafkaProvider { 
    KAFKACONN.get().expect("Missing Session for Kafka")
}
pub struct KafkaProvider(pub FutureProducer);

// Create the `FutureProducer` to produce asynchronously.
#[tracing::instrument(level = "debug")]
pub fn create_producer() -> FutureProducer {
    log::info!("🚧🚧 Running Kafka Producer at {}", KAFKA_BROKER.as_str());

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKER.as_str())
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failed");
    let _ = KAFKACONN.set(KafkaProvider::from(producer.clone()));
    producer
}

impl From<FutureProducer> for KafkaProvider { 
    fn from(f: FutureProducer) -> Self {
        Self(f)
    }
}

// Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
#[tracing::instrument(level = "debug")]
pub fn create_consumer(group_id: String) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", KAFKA_BROKER.as_str())
        .set("enable.partition.eof", "false")
        .set("auto_offset_reset", "earliest")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&KAFKA_TOPIC])
        .expect("Can't subscribe to specified topics");
    consumer
}

// pub fn get_kafka_consumer_group_id(kafka_consumer_counter: &Mutex<i32>) -> String {
//     let mut counter = kafka_consumer_counter.lock().expect("Can't lock counter");
//     *counter += 1;
//     format!("graphql-group-{}", *counter)
// }
#[tracing::instrument(level = "debug")]
pub async fn send_message(message: &str) {
    let futures: Vec<_> = (0..5).map(|_| async move { 
        let delivery_status = kafka_producer()
            .0
            .send(
                FutureRecord::to(&KAFKA_TOPIC)
                .payload(message)
                .key(MESSAGE_KEY.as_str()),
                Timeout::After(Duration::from_secs(0)),
            )
            .await;
            log::info!("Delivery Status {:?} for {:#?}", delivery_status, message);
            delivery_status
    }).collect();    

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        let delivery_status = future.await;
        match delivery_status {
            Ok((a, b)) => {
                log::info!("👏👏 Success message is sent! {}, {}", a, b);
            }
            Err((e, _)) => {
                log::info!("❌❌ Something went wrong: error {}", e);
            }
        }
    }
}