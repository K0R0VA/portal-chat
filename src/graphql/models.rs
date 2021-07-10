use async_graphql::{Object, Context, Result};
use async_graphql::dataloader::DataLoader;
use std::ops::Deref;
use crate::storage::Storage;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UserId(pub i32);
impl Deref for UserId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[Object]
impl User {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx.data_unchecked::<DataLoader<Storage>>();
        let user = loader.load_one(UserId(self.id)).await?;
        user.ok_or_else(|| "Not found".into())
    }
}