pub mod query;
pub mod mutation;
pub mod loader;
mod models;

use actix::{Addr, Actor};
use async_graphql::{Schema, EmptySubscription};

use crate::actors::state::State;
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::storage::Storage;


pub type DefaultSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(chat_server: Addr<State>) -> DefaultSchema {
    let storage = Storage::default();
    Schema::build(Query, Mutation, EmptySubscription)
        .data(storage.clone())
        .data(storage.start())
        .data(chat_server)
        .finish()
}