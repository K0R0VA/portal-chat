use deadpool_postgres::Pool;
use crate::messages::ws_messages::{UserChatsIds};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;

pub struct StateStorage {
    pub pool: Pool,
}

#[async_trait::async_trait]
impl Loader<i32> for StateStorage {
    type Value = UserChatsIds;
    type Error = ();

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let result: anyhow::Result<HashMap<i32, UserChatsIds>> = try {
            let client = self.pool.get().await?;
            let rooms_stmt =
                client.prepare(
                    "select room_id from participant where user_id = $1").await?;
            let contacts_stmt =
                client.prepare("select contact_id from contact where user_id = $1 or contact_id = $2")
                    .await?;
            let (rooms, contacts) = futures::future::try_join(
                client.query(&rooms_stmt, &[&keys[0]]),
                client.query(&contacts_stmt, &[&keys[0]]),
            ).await?;
            vec![(keys[0], UserChatsIds {
                contacts: serde_postgres::from_rows(&contacts)?,
                rooms: serde_postgres::from_rows(&rooms)?,
            })]
                .into_iter()
                .collect()
        };
        result.map_err(|_| ())
    }
}
