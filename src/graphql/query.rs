use super::models::user::{User, UserId};

use async_graphql::{Object, Context, Result};
use async_graphql::dataloader::DataLoader;
use itertools::Itertools;
use crate::graphql::loaders::user_loader::UserLoader;
use crate::graphql::models::room_message::RoomMessage;
use crate::graphql::loaders::room_message_loader::RoomMessagesLoader;
use crate::graphql::models::room::RoomId;

pub struct Query;

#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<User, String> {
        let fields = ctx
            .field()
            .selection_set()
            .filter(|field| match field.name() {
                "rooms" | "friends" | "id" => false,
                _ => true
            } )
            .map(|field| field.name())
            .join(",");
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        let user = loader.load_one(UserId(id)).await?;
        user.ok_or_else(|| "Not found".into())
    }
    async fn room_messages(&self, ctx: &Context<'_>, id: i32)
        -> Result<Option<Vec<RoomMessage>>, String> {
        let loader = ctx.data_unchecked::<DataLoader<RoomMessagesLoader>>();
        let messages = loader.load_one(RoomId(id)).await?;
        Ok(messages)
    }
}