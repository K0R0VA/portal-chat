use async_graphql::{Object, Context, Result};
use itertools::Itertools;
use async_graphql::dataloader::DataLoader;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::graphql::models::user::{User, UserId};
use crate::graphql::loaders::user_loader::UserLoader;


#[derive(Deserialize, Clone)]
pub struct RoomMessage {
    pub id: Option<i32>,
    pub send_date: Option<NaiveDateTime>,
    pub room_id: i32,
    pub sender_id: i32,
    pub content: Option<String>,
}

#[Object]
impl RoomMessage {
    async fn sender_id(&self) -> i32 {
        self.sender_id
    }
    async fn content(&self) -> Option<&str> {
        Option::as_deref(&self.content)
    }
    async fn sender(&self, ctx: &Context<'_>) -> Result<User> {
        let fields = ctx
            .field()
            .selection_set()
            .filter(|field| match field.name() {
                "rooms" | "friends" | "id" => false,
                _ => true
            })
            .map(|field| field.name())
            .join(",");
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        let sender_id = self.sender_id;
        let user = loader.load_one(
            UserId(sender_id, fields))
            .await
            .unwrap();
        user.ok_or_else(|| "Not found".into())
    }
}