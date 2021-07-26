pub mod query;
pub mod mutation;
pub mod models;
pub mod loaders;
mod inputs;

use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;

use async_graphql::{Schema, EmptySubscription, SchemaBuilder};


pub type DefaultSchema = Schema<Query, Mutation, EmptySubscription>;
pub type Builder = SchemaBuilder<Query, Mutation, EmptySubscription>;

