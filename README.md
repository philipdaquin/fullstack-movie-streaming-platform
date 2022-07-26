# Fullstack Movie Streaming Platform
📺 Netflix in RUST/ NextJS, Actix-Web, Async Apollo-GraphQl, Cassandra/ ScyllaDB, Async SQLx, Spark, Kafka, Redis, Elasticsearch, Influxdb Iox, Tensorflow

### System Design Prompt: Design Netflix 
##### Disclaimer: The project was intended to build a system like Netflix/ Youtube/ Tiktok from PoC to Production Like environment. Since I do not own the rights for any of the movies, I decided to replace the movie 'links' with Youtube Trailers instead. 

### This is not the official repository. Original repo: 330 + commits

Design a video on demand platform that lets users upload fast, watch and store videos like Tiktok, Youtube and Amazon Prime. Build the system so that it is always available for our users, highly scalable, high performance and low latency, cost effective and supported by different devices.

## Goals & Tasks
User goals: 
1.    Watch their favourite show seamlessly 
2.    Wide range of choices to pick from 
3.    Seamless membership experience, which they cancel anytime  
4.    Save favourite videos so they can watch it later 

User Task: 
1.    Sign up and pick a subscription plan 
2.    Search/ browse their favourite shows 
3.    Choose from a wide selection of videos
4.    Watch movie

## Tech stack
### Client 
- NextJS/React/TypeScript
- Tailwind CSS
- CSS + HTML
- Stripe Payments
- Firebase
 ### Distributed System Tech Stack+ 
- Apollo-Federation Router (Rust)
- Async GraphQL 
- Actix-Web
- Redis
- Docker / Kubernetes (wip)
- Async SQLx + PostgreSQL
- ScyllaDB/ CassandraDB 
- Apache Kafka
- ~~Apache Spark (pySpark)~~ (wip w/ tensorflow)
- ~~Hadoop(HDFS)~~ 
- AWS WORKFLOW
- Elastic Stack (Elasticsearch, Kabana)
- ~~Apache Beam and BigQuery~~ 
- Influx DB
- Tensorflow/ TFX
- ~~BigQuery ML | Dataflow~~  

### General Logging Stack  
- Prometheus
- Open Telemetry
- Grafana
- Jaeger

![homepage](https://user-images.githubusercontent.com/85416532/180874040-20bc8939-52b6-458f-834a-70f9d19a7665.png)

## Final Architecture
![architecture_final](https://user-images.githubusercontent.com/85416532/180871640-580399e9-0070-42f5-8a29-5d4a2472ce53.png)
- Build logs// Notes// Brainstorming of ideas/ Challenges can be found [here](https://github.com/philipdaquin/Fullstack-Movie-Streaming-Platform/tree/main/Notes) 

## Subscription Plan
![subs](https://user-images.githubusercontent.com/85416532/180876809-a91f4bf6-9b67-4543-b06f-f95cfaafda96.png)

## Search 
![search_page](https://user-images.githubusercontent.com/85416532/180875454-17dbda3d-3b19-4b5b-a345-8b9f6759f5e9.png)
![search_page1](https://user-images.githubusercontent.com/85416532/180875600-920e40d6-a0ef-4067-8c0b-1622aac5c55a.png)

## Profile Page 
![profile](https://user-images.githubusercontent.com/85416532/180876453-70257407-efd9-4324-bd45-3a58d7c8ab39.png)

## Sign Up/ Sign In
![image](https://user-images.githubusercontent.com/85416532/180876561-79cacac6-338b-4036-9e3b-93a804bd9f5c.png)

## Initial Architecture 
[
![architecture_sample](https://user-images.githubusercontent.com/85416532/179338067-ba374ff4-2825-4bff-a4a7-a0ae83085366.png)
](url)


