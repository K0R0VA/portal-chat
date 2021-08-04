use deadpool_postgres::{Pool, Client};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use itertools::join;

use crate::graphql::models::user::User;
use crate::graphql::models::room::RoomId;
use crate::graphql::loaders::LoadResult;

pub struct ParticipantsLoader(pub Pool);

#[async_trait::async_trait]
impl Loader<RoomId> for ParticipantsLoader {
    type Value = Vec<User>;
    type Error = String;

    async fn load(&self, keys: &[RoomId]) -> Result<HashMap<RoomId, Self::Value>, Self::Error> {
        let result: LoadResult<RoomId, Self::Value> = try {
            let pool = &self.0;
            let client: Client = pool.get().await?;
            let stmt = client.prepare(&*format!(
                r#"select participant.room_id, json_agg(
	                json_build_object(
		                'id', public.user.id,
		                'name', public.user.name,
		                'avatar', public.user.avatar,
	                )
                ) as "participants"
                from public.user
	            right join participant on  participant.participant_id = public.user.id
	                where room_id in ({})
	            group by participant.room_id;"#,
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
                    let participants: Vec<User> = serde_json::from_value(json).expect("Bad sql query");
                    (
                        RoomId(room_id),
                        participants
                    )
                })
                .collect()
        };
        result.map_err(|e| e.to_string())
    }
}