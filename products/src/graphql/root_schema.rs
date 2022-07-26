use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
    use super::modules::schema::{QueryProducts, MutateProduct};


#[derive(MergedObject, Default)]
pub struct Query(QueryProducts);

#[derive(MergedObject, Default)]
pub struct Mutation(MutateProduct);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
