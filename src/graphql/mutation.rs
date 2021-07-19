use async_graphql::{Object, Context, Result};
use crate::messages::db_messages::{SignUp};
use crate::storage::Storage;
use actix::Addr;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_up(&self, ctx: &Context<'_>, name: String) -> Result<i32> {
        let storage = ctx.data_unchecked::<Addr<Storage>>();
        let id = storage.send(SignUp { name }).await.unwrap();
        Ok(id)
    }
}