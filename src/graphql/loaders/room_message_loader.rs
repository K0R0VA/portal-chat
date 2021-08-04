use deadpool_postgres::{Pool, Client};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use itertools::join;

use crate::graphql::models::room_message::RoomMessage;
use crate::graphql::models::room::RoomId;
use crate::graphql::loaders::LoadResult;


pub struct RoomMessagesLoader(pub Pool);

#[async_trait::async_trait]
impl Loader<RoomId> for RoomMessagesLoader {
    type Value = Vec<RoomMessage>;
    type Error = String;

    async fn load(&self, keys: &[RoomId]) -> Result<HashMap<RoomId, Self::Value>, Self::Error> {
        let result: LoadResult<RoomId, Self::Value> = try {
            let pool = &self.0;
            let client: Client = pool.get().await?;
            let stmt = client.prepare(&*format!(
                r#"select id, json_agg(json_build_object(
                    'sender_id', sender_id,
                    'room_id', room_id,
                    'msg', msg,
                    'replayed_message_id', replayed_message_id)
                    ) as "room_messages" from room_message
                    where room_id in ({}) group by room_id"#,
                join(keys
                         .into_iter()
                         .map::<i32, _>(|room| room.0),
                     ",")
            )).await?;
            let result = client.query(&stmt, &[]).await?;
            result
                .iter()
                .map(|row| {
                    let room_id: i32 = row.get(0);
                    let json = row.get::<usize, serde_json::Value>(1);
                    let messages: Vec<RoomMessage> =
                        serde_json::from_value(json).expect("Bad sql query");
                    (
                        RoomId(room_id),
                        messages
                    )
                })
                .collect()
        };
        result.map_err(|e| e.to_string())
    }
}