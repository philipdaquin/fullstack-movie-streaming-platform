use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::types::{
    ProductionCompanyQuery, ProductionCompanyMutation,
    MovieMutation, PersonQuery, PersonMutation
};

#[derive(MergedObject, Default)]
pub struct Query(ProductionCompanyQuery, PersonQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ProductionCompanyMutation, MovieMutation, PersonMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
