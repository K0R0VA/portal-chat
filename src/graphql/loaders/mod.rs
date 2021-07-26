use std::collections::HashMap;
use crate::graphql::Builder;
use deadpool_postgres::Pool;
use crate::graphql::loaders::participants_loader::ParticipantsLoader;
use async_graphql::dataloader::DataLoader;
use crate::graphql::loaders::room_message_loader::RoomMessagesLoader;
use crate::graphql::loaders::user_loader::UserLoader;
use crate::graphql::loaders::rooms_loader::RoomsLoader;

pub mod user_loader;
pub mod rooms_loader;
pub mod participants_loader;
pub mod room_message_loader;

pub type LoadResult<K, T> = anyhow::Result<HashMap<K, T>>;

pub fn setup_loaders(builder: Builder, pool: Pool) -> Builder {
    let participants_loader = DataLoader::new(
        ParticipantsLoader(pool.clone())
    );
    let room_message_loader = DataLoader::new(
        RoomMessagesLoader(pool.clone())
    );
    let user_loader = DataLoader::new(
        UserLoader(pool.clone())
    );
    let rooms_loader = DataLoader::new(
        RoomsLoader(pool.clone())
    );
    builder
        .data(participants_loader)
        .data(room_message_loader)
        .data(rooms_loader)
        .data(user_loader)
}