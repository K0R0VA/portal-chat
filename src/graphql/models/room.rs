use async_graphql::{Object, Context, Result};
use serde::Deserialize;

use crate::graphql::models::user::User;
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
pub struct RoomId(pub i32);


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
        let loader = ctx
            .data_unchecked::<DataLoader<ParticipantsLoader>>();
        let participants = loader.load_one(RoomId(self.id)).await?;
        participants.ok_or_else(|| "Not found any participants".into())
    }
    async fn messages(&self, ctx: &Context<'_>) -> Result<Option<Vec<RoomMessage>>> {
        let loader = ctx
            .data_unchecked::<DataLoader<RoomMessagesLoader>>();
        let messages = loader.load_one(RoomId(self.id)).await?;
        Ok(messages)
    }

}


