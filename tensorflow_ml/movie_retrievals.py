# !pip install -q tensorflow-recommenders
# !pip install -q --upgrade tensorflow-datasets
# !pip install -q scann
# pip install tensorflow-io
# pip install kafka-python

# Tensorflow
import os
import pprint
import tempfile
from typing import Dict, Text
import numpy as np
from json import loads
import tensorflow as tf
import tensorflow_recommenders as tfrs
import tensorflow_datasets as tfds
import tensorflow_io as tfio 
from kafka import KafkaProducer, KafkaConsumer
from kafka.errors import KafkaError
# from sklearn import train_test_split
import pandas as pd 


# Data Retrieval from the User Activity Logs
KAFKA_GROUP_ID='user_recommendation'
KAFKA_TOPIC_NAME_MOVIES = "movie_topic"
KAFKA_TOPIC_NAME_USER = "user_analytics"
KAFKA_CLIENT = "localhost:9092"
KAFKA_PRODUCER = "localhost:9093"
KAFKA_PRODUCER_TOPIC = "recommended_movies"
# Consumer for Movies Dataset
# When a new movie is added, the mvoie is added to a queue
movie_consumer = KafkaConsumer(
    KAFKA_TOPIC_NAME_MOVIES,
    bootstrap_servers=[KAFKA_CLIENT],
    enable_auto_commit=True,
    group_id=KAFKA_GROUP_ID,
    auto_offset_reset='earliest',
    # auto_commit_interval_ms=5000,
    session_timeout_ms=6000,
    # Decode the message comming from the producer
    value_deserializer=lambda x: loads(x.decode('utf-8'))
)
# Consumer for User Analytics Service
user_rating_consumer = KafkaConsumer(
    KAFKA_TOPIC_NAME_USER,
    bootstrap_servers=[KAFKA_CLIENT],
    enable_auto_commit=True,
    group_id=KAFKA_GROUP_ID,
    auto_offset_reset='earliest',
    session_timeout_ms=6000,
    value_deserializer=lambda x: loads(x.decode('utf-8'))
)

# User Ratings
# Consume messages from User Activity
# The ratings dataset returns a dictionary of movie id, user id, the assigned
# rating, timestamp, movie information, and user information
for msg in user_rating_consumer:
    kafka_message = msg.value
    print(kafka_message)
    # ratings = tfds.load(kafka_message, split="train")
# Movies data.
# ratings = tfds.load("movielens/100k-ratings", split="train")
# Features of all the available movies.
# The movies dataset contains the movie id, movie title and data on what
# genres it belongs to.
for msg in movie_consumer:
    movies = msg.value

movies = tf.data.Dataset.from_tensor_slices(dict(movies)).map(lambda x: { x["title"]}).batch(4)
ratings = tf.data.Dataset.from_tensor_slices(dict(kafka_message)).batch(4)
# To fit and evaluate the mdoel, we need to split it into a training and evaluation set
tf.random.set_seed(42)
shuffled = ratings.shuffle(100_000, seed=42, reshuffle_each_iteration=False)
train = shuffled.take(80_000)
test = shuffled.skip(80_000).take(20_000)

# We need a vocabulary that maps a raw feature value to an integer in a contiguous range: this
# allows us to look up the corresponding embeddings in our embedding tables
movie_titles = movies.batch(1_000)
user_ids = ratings.batch(1_000_000).map(lambda x: x["user_id"])
unique_movie_titles = np.unique(np.concatenate(list(movie_titles)))
unique_user_ids = np.unique(np.concatenate(list(user_ids)))

# unique_movie_titles[:10]
# Higher values will correspond to models that may be more accureate, but will
# also be slower to fit and more prone to overfitting
EMBEDDING_DIMENSION = 32

# Query Tower
# Define the model itself
# Convert user ids to integers, and then convert those to use embeddings via an
# Embedding layer
user_model = tf.keras.Sequential([
  tf.keras.layers.StringLookup(
      vocabulary=unique_user_ids, mask_token=None),
  # We add an additional embedding to account for unknown tokens.
  tf.keras.layers.Embedding(len(unique_user_ids) + 1, EMBEDDING_DIMENSION)
])
# The Candidate Tower
movie_model = tf.keras.Sequential([
  tf.keras.layers.StringLookup(
      vocabulary=unique_movie_titles, mask_token=None),
  tf.keras.layers.Embedding(len(unique_movie_titles) + 1, EMBEDDING_DIMENSION)
])
print(movie_model)
metrics = tfrs.metrics.FactorizedTopK(candidates=movies.batch(128).map(movie_model))
task = tfrs.tasks.Retrieval(metrics=metrics)
print(metrics)
print(task)

class MovielensModel(tfrs.Model): 
  def __init__(self, user_model, movie_model):
    super().__init__()
    self.movie_model: tf.keras.Model = movie_model
    self.user_model: tf.keras.Model = user_model
    self.task: tf.keras.layers.Layer = task

  def compute_loss(self, features: Dict[Text, tf.Tensor], training=False) -> tf.Tensor:
    # We pick out the user features and pass them into the user model.
    user_embeddings = self.user_model(features["user_id"])
    # And pick out the movie features and pass them into the movie model,
    # getting embeddings back.
    print(user_embeddings)
    positive_movie_embeddings = self.movie_model(features["title"])
    # The task computes the loss and the metrics.
    return self.task(user_embeddings, positive_movie_embeddings)

class NoBaseClassMovieLensModel(tf.keras.Model):
  def __init__(self, user_model, movie_model):
    super().__init__()
    self.movie_model: tf.keras.Model = movie_model
    self.user_model: tf.keras.Model = user_model 
    self.task: tf.keras.layers.Layer =  task
  def train_step(self, features: Dict[Text, tf.Tensor]) -> tf.Tensor:
    # Set up a gradient tape to record gradients
    with tf.GradientTape() as tape:
      user_embeddings = self.user_model(features["user_id"])
      positive_movie_embeddings = self.movie_model(features["title"])
      loss = self.task(user_embeddings, positive_movie_embeddings)
      regularisation_loss = sum(self.losses)
      total_loss = loss + regularisation_loss
    gradients = tape.gradient(total_loss, self.trainable_variables)
    self.optimizer.apply_gradient(zip(gradients, self.trainable_variables))
    metrics = {metric.name: metric.result() for metric in self.metrics} 
    metrics["total_loss"] = total_loss
    metrics["loss"] = loss
    metrics["regularisation_loss"] = regularisation_loss
    print(metrics)
    return metrics 

  def test_step(self, features: Dict[Text, tf.Tensor]) -> tf.Tensor:
    # loss computation 
    user_embeddings = self.user_model(features["user_id"])
    positive_movie_embeddings = self.movie_model(features["title"])
    loss = self.task(user_embeddings, positive_movie_embeddings)

    # handle regularisaton losses as well 
    regularisation_loss = sum(self.losses)
    total_loss = loss + regularisation_loss
    metrics = {metric.name: metric.result() for metric in self.metrics} 
    metrics["total_loss"] = total_loss
    metrics["loss"] = loss
    metrics["regularisation_loss"] = regularisation_loss
    return metrics

model = MovielensModel(user_model, movie_model)
model.compile(optimizer = tf.keras.optimizers.Adagrad(learning_rate = 0.1))

cached_train = train.shuffle(100_000).batch(8192).cache()
cached_test = test.batch(4096).cache()

model.fit(cached_train, epochs=3)

model.evaluate(cached_test, return_dict = True)

# Making Predictions 
# Create a model that takes in raw query features and 
index = tfrs.layers.factorized_top_k.BruteForce(model.user_model)

# Recommends movies out of the entire movies dataset
recommended_movies = index.index_from_dataset(tf.data.Dataset.zip(
    (movies.batch(100), movies.batch(100).map(model.movie_model))))


# Export the query model.
with tempfile.TemporaryDirectory() as tmp:
  path = os.path.join(tmp, "model")
  # Save the index.
  tf.saved_model.save(
      index,
      path,
      options=tf.saved_model.SaveOptions(namespace_whitelist=["Scann"])
  )
  # Load it back; can also be done in TensorFlow Serving.
  loaded = tf.saved_model.load(path)
  # Pass a user id in, get top predicted movie titles back.
  scores, titles = loaded(["42"])
  print(f"Recommendations: {titles[0][ : 3]}")

# Kafka Producer
# Store the train and send new messages to kafka
def error_callback():
  raise Exception('Error while sending data to kafka!')

# Write to a seperate messaging queue using 'LocalHost:9093'
def write_to_kafka(topic_name, items):
  count = 0
  producer = KafkaProducer(bootstrap_servers=[KAFKA_PRODUCER])
  for msg, key in items:
    producer \
      .send(topic_name, key=key.encode('utf-8'), value=msg.encode('utf-8')) \
      .add_errback(error_callback)
    count +=1
  producer.flush()
  print(f'Wrote {0} messages into topic: {1}'.format(count, topic_name))

# Write to kafka
# Send over the entire dataset
write_to_kafka(KAFKA_PRODUCER_TOPIC, recommended_movies)