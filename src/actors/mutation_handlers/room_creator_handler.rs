use deadpool_postgres::{Pool, Client};
use actix::{Actor, Context, Handler, ResponseFuture};
use crate::messages::db_messages::CreateRoom;
use crate::graphql::models::room::Room;
use itertools::Itertools;

pub struct RoomCreatorHandler {
    pub pool: Pool,
}

impl Actor for RoomCreatorHandler {
    type Context = Context<Self>;
}

impl Handler<CreateRoom> for RoomCreatorHandler {
    type Result = ResponseFuture<anyhow::Result<Room>>;

    fn handle(&mut self, msg: CreateRoom, _: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            try {
                let client: Client = pool.get().await?;
                let participants = msg.participants.map(|contacts| {
                    contacts
                        .into_iter()
                        .map(|contact_id| format!("({}, (select id from create_room))", contact_id))
                        .join(",")
                }).unwrap_or("".to_string());
                let stmt = client.prepare(
                    "with create_room as (insert into room (name, avatar, creator_id)
					        values($1, $2, $3) returning *),
                            push_participants as (insert into participant (user_id, room_id)
 	                            values
	                            ($3, (select id from create_room)),
                                $4
                            )
                           select * from create_room"
                )
                    .await?;
                let row =
                    client.query_one(&stmt, &[&msg.name, &msg.avatar, &msg.creator_id, &participants])
                        .await?;
                serde_postgres::from_row::<Room>(&row)?
            }
        })
    }
}