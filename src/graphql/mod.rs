pub mod query;
pub mod mutation;
pub mod models;
mod loaders;
mod inputs;

use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;

use async_graphql::{Schema, EmptySubscription};


pub type DefaultSchema = Schema<Query, Mutation, EmptySubscription>;

