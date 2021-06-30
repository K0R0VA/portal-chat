use actix::{Message};

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateRoom {
    creator_id: i32,
    name: String,
    participants: Option<Vec<i32>>
}

