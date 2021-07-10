use actix::{Actor, Context};
use crate::storage::Storage;

impl Actor for Storage {
    type Context = Context<Self>;
}