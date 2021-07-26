use deadpool_postgres::{Pool, Client};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use itertools::join;

use crate::graphql::models::room::Room;
use crate::graphql::models::user::ParticipantId;
use crate::graphql::loaders::LoadResult;


pub struct RoomsLoader(pub Pool);

#[async_trait::async_trait]
impl Loader<ParticipantId> for RoomsLoader {
    type Value = Vec<Room>;
    type Error = String;

    async fn load(&self, keys: &[ParticipantId]) -> Result<HashMap<ParticipantId, Self::Value>, Self::Error> {
        let result: LoadResult<ParticipantId, Self::Value> = try {
            let pool = &self.0;
            let client: Client = pool.get().await?;
            let fields = keys[0].1.as_str();
            let stmt = client.prepare(&*format!(
                r#"select room_id, json_agg(json_build_object({})) as "room_messages"
                    from room_message where room_id in ({}) group by room_id"#,
                fields,
                join(keys
                         .into_iter()
                         .map::<i32, _>(|participant| participant.0),
                     ",")
            )).await?;
            let result = client.query(&stmt, &[]).await?;
            result
                .iter()
                .map(|row| {
                    let participant_id: i32 = row.get(0);
                    let json = row.get::<usize, serde_json::Value>(1);
                    let rooms: Vec<Room> = serde_json::from_value(json).expect("Bad sql query");
                    (
                        ParticipantId(participant_id, fields.to_string()),
                        rooms
                    )
                })
                .collect()
        };
        result.map_err(|e| e.to_string())
    }
}
