use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::{ElasticQuery, ElasticMutate};

#[derive(MergedObject, Default)]
pub struct Query(ElasticQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ElasticMutate);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
