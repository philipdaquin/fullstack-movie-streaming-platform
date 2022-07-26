use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::user_module::schema::{UserQuery, UserMutation};
use super::profile_module::schema::{ProfileQuery, ProfileMutation};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, ProfileQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, ProfileMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
