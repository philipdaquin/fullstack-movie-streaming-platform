use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::{AnalyticsMutation, AnalyticsQuery};

#[derive(MergedObject, Default)]
pub struct Query(AnalyticsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AnalyticsMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
