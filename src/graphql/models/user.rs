use async_graphql::{Object, Context, Result};
use async_graphql::dataloader::{DataLoader};
use itertools::Itertools;
use serde::Deserialize;

use crate::graphql::models::room::Room;
use crate::graphql::loaders::rooms_loader::RoomsLoader;
use chrono::NaiveDate;


#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UserId(pub i32);

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ParticipantId(pub i32);








#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub last_session: Option<NaiveDate>,
}

#[Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> Option<&str> {
        Option::as_deref(&self.name)
    }

    async fn avatar(&self) -> Option<&str> {
        Option::as_deref(&self.avatar)
    }

    async fn rooms(&self, ctx: &Context<'_>) -> Result<Option<Vec<Room>>> {
        let loader = ctx.data_unchecked::<DataLoader<RoomsLoader>>();
        let rooms = loader.load_one(ParticipantId(self.id)).await?;
        Ok(rooms)
    }
    async fn friends(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        Ok(vec![])
    }
}