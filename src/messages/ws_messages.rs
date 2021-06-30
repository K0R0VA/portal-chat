use actix::{Message, Recipient};
use uuid::Uuid;
use std::sync::Arc;

pub type Socket = Recipient<WsMessage>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Socket,
    pub lobby_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectToRoom {
    pub addr: Arc<Socket>,
    pub sender_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectFromRoom {
    pub socket: Arc<Socket>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub sender_id: Uuid,
    pub room_id: Uuid,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PrivateMessage {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateRoom {
    pub id: Uuid,
    pub creator_id: Uuid,
}


