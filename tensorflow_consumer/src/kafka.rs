use std::sync::Mutex;
use futures::StreamExt;
use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer, CommitMode};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, ClientContext, Message};
use once_cell::sync::OnceCell;

use crate::db;
use crate::modules::model::{NewRecommendedMovies, RecommendedMovies};

lazy_static! {
    pub static ref KAFKA_BROKER: String = std::env::var("KAFKA_BROKER").expect("Can't read Kafka broker address");
    static ref KAFKA_TOPIC: String = std::env::var("KAFKA_TOPIC").expect("Can't read Kafka topic name");
    static ref MESSAGE_KEY: String = std::env::var("MESSAGE_KEY").expect("Unable to get a valid message key");
    static ref CONSUMER_GROUP_ID: String = std::env::var("CONSUMER_GROUP_ID").expect("Expected a valid group id for consumers");
}

pub static KAFKACONN: OnceCell<KafkaClientContext> = OnceCell::new();

#[inline]
pub(crate) fn kafka_client() -> &'static KafkaClientContext { 
    KAFKACONN.get().expect("Missing Kafka Client")
}
/// A context can be used to change the behaviour of producers and consumers by adding callbacks 
/// that will be executed by librdkafka. This particular context sets up custom callbacks to log 
/// replacing events 
pub struct KafkaClientContext(pub StreamConsumer);
impl From<StreamConsumer> for KafkaClientContext { 
    fn from(f: StreamConsumer) -> Self {
        Self(f)
    }
}
#[derive(Debug, Clone)]
pub struct MessagePayload(String);

impl MessagePayload {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// generic way to turn a borrowed message into a (wrapped) string
impl<'a> From<&'a BorrowedMessage<'a>> for MessagePayload {
    fn from(bm: &'a BorrowedMessage) -> Self {
        match bm.payload_view::<str>() {
            Some(Ok(s)) => MessagePayload(String::from(s)),
            Some(Err(e)) => MessagePayload(format!("{:?}", e)),
            None => MessagePayload(String::from("")),
        }
    }
}

// Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
#[tracing::instrument(level = "debug", err)]
pub fn create_consumer_dual_writes() -> Result<(), KafkaError> {
    log::info!("🛰️ Running Kafka Consumer at {}", KAFKA_BROKER.as_str());

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", CONSUMER_GROUP_ID.as_str())
        .set("bootstrap.servers",  KAFKA_BROKER.as_str())
        .set("auto.offset.reset", "latest")
        .set("enable.partition.eof", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //  Commit every 5 seconds
        .set("auto.commit.interval.ms", "5000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&[&KAFKA_TOPIC])
        .expect("Can't subscribe to specified topics");
    let _ = KAFKACONN.set(KafkaClientContext::from(consumer));
    
    Ok(())
}
/// Consumer group for the Asset Service where the payload 
///  is sent to ElasticSearch to be indexed
#[tracing::instrument(level = "debug")]
pub async fn run_consumer_group_dual_writes() -> Result<(), KafkaError> { 
    //  Create Kafka Consumer
    log::info!("🚦 Spawning consumer group: {}", CONSUMER_GROUP_ID.as_str());
    let stream = kafka_client();
    loop {
        match stream.0.recv().await {
            Err(e) => {
                log::warn!("Kafka error: {}", e);
            }
            
            Ok(message) => {
                let payload = MessagePayload::from(&message);
                log::info!("{:#?}", payload);
                let mut index_movies: Vec<_> = Vec::new();
                log::info!("👷‍♂️👷‍♂️ key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    message.key(), 
                    payload, 
                    message.topic(), 
                    message.partition(), 
                    message.offset(), 
                    message.timestamp());
                
                log::info!("MOVIE STRUCT {:#?}", index_movies);
                // Pass this dataset to Elasticsearch to be indexed 
                log::info!("🛬 Received Payload from {}, Sending it over to Elasticsearch Cluster ", KAFKA_BROKER.as_str());
                
                // Add 'created_at' row to the payload so we can sort the messages for the user 
                let new_recommendations: NewRecommendedMovies = serde_json::from_str(payload.as_str()).expect("Something went wrong in the payoad");
                //  Convert NewRecommendations with 'created_at' column
                index_movies.push(RecommendedMovies::from(new_recommendations));
                //  Write new recommendations into database
                db::stream_insert(index_movies, db::session())
                    .await
                    .expect("Unable to index the movie at {payload:?}");
            }
        };
    }
}