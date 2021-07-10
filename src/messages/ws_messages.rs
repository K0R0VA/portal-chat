use actix::{Message, Addr};
use serde::Deserialize;
use crate::actors::web_socket::WebSocket;


#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WebSocket>,
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectToRoom {
    pub addr: Addr<WebSocket>,
    pub sender_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectFromRoom {
    pub socket_id: i32,
}

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub sender_id: i32,
    pub room_id: i32,
    pub msg: String,
}

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct PrivateMessage {
    pub id: i32,
    pub recipient_id: i32,
    pub msg: String,
}

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct CreateRoom {
    pub id: i32,
    pub creator_id: i32,
}


