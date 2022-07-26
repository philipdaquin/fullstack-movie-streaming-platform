use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::RecommendedQuery;
#[derive(MergedObject, Default)]
pub struct Query(RecommendedQuery);

#[derive(MergedObject, Default)]
pub struct Mutation;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, EmptyMutation, EmptySubscription>;
