use actix::{Message, Addr};
use serde::Deserialize;
use uuid::Uuid;

use crate::actors::session::{Session};
use crate::actors::user::User;
use crate::actors::room::Room;


#[derive(Message)]
#[rtype(result = "anyhow::Result<Vec<i32>>")]
pub struct GetUserRooms(pub i32);

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomIsEmpty(pub i32);

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewSession {
    pub session_id: Uuid,
    pub session: Addr<Session>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseSession(pub Uuid);

#[derive(Deserialize)]
pub struct Connect {
    pub user_id: i32,
    pub rooms: Vec<i32>,
    pub friends: Vec<i32>
}

impl Message for Connect {
    type Result = Addr<User>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectToRoom {
    pub session: Addr<Session>,
    pub session_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectFromRoom {
    pub session_id: Uuid,
}

#[derive(Message, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub sender_id: i32,
    pub room_id: i32,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PrivateMessageToContact {
    pub contact_id: i32,
    pub msg: String,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PrivateMessage {
    pub user_id: i32,
    pub contact_id: i32,
    pub msg: String,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct AddContact {
    pub user_id: i32
}

#[derive(Message)]
#[rtype(result = "Option<Addr<User>>")]
pub struct GetUser {
    pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct CreateRoom {
    pub id: i32,
    pub creator_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewRoom {
    pub id: i32,
    pub room: Addr<Room>
}


