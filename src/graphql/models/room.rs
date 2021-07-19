use async_graphql::{Object, Context, Result};
use serde::Deserialize;

use crate::graphql::models::user::{User};
use itertools::Itertools;
use async_graphql::dataloader::DataLoader;
use crate::graphql::models::room_message::RoomMessage;
use crate::graphql::loaders::participants_loader::ParticipantsLoader;
use crate::graphql::loaders::room_message_loader::RoomMessagesLoader;

#[derive(Clone, Deserialize)]
pub struct Room {
    pub(crate) id: i32,
    pub(crate) name: Option<String>,
    pub(crate) avatar: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RoomId(pub i32, pub String);


#[Object]
impl Room {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> Option<&str> { Option::as_deref(&self.name) }
    async fn avatar(&self) -> Option<&str> {
        Option::as_deref(&self.avatar)
    }
    async fn participants(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let fields = ctx
            .field()
            .selection_set()
            .filter(|field| match field.name() {
                "rooms" | "friends" | "id" => false,
                _ => true
            } )
            .map(|field| ["'", field.name() ,"', public.user.", field.name()].concat())
            .join(",");
        let loader = ctx
            .data_unchecked::<DataLoader<ParticipantsLoader>>();
        let participants = loader.load_one(RoomId(self.id, fields)).await?;
        participants.ok_or_else(|| "Not found any participants".into())
    }
    async fn messages(&self, ctx: &Context<'_>) -> Result<Vec<RoomMessage>> {
        let fields = ctx
            .field()
            .selection_set()
            .filter(|field|  match field.name() {
                "user" | "id" => false,
                _ => true
            })
            .map(|field| field.name())
            .join(",");
        let loader = ctx
            .data_unchecked::<DataLoader<RoomMessagesLoader>>();
        let messages = loader.load_one(RoomId(self.id, fields)).await?;
        messages.ok_or_else(|| "Не было найдено ни одного сообщения".into())
    }

}


