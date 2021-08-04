use actix::{Message, Addr, Recipient};
use serde::Deserialize;
use uuid::Uuid;

use crate::actors::session::{Session};
use crate::actors::user::User;
use crate::actors::room::Room;
use std::collections::HashMap;


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
}

impl Message for Connect {
    type Result = Session;
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<UserChats>")]
pub struct GetUserChatsIds {
    pub user_id: i32
}

#[derive(Clone)]
pub struct UserChatsIds {
    pub contacts: Vec<i32>,
    pub rooms: Vec<i32>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GetUserChats {
    pub user_address: Recipient<UserChats>,
    pub user_id: i32
}


#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserChats {
    pub contacts: HashMap<i32, Addr<User>>,
    pub rooms: HashMap<i32, Addr<Room>>
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
pub struct AddContactToUser {
    pub user_id: i32,
    pub contact_id: i32
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct AddContactActor {
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

