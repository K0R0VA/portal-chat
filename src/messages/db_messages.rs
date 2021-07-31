use actix::Message;
use anyhow::Result;

use crate::graphql::models::user::User;
use crate::graphql::models::room::Room;

#[derive(Message)]
#[rtype(result = "Result<User>")]
pub struct SignUp {
    pub name: String,
    pub password: String
}

#[derive(Message)]
#[rtype(result = "Result<User>")]
pub struct SignIn {
    pub name: String,
    pub password: String
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct Logout {
    pub user_id: i32
}

#[derive(Message)]
#[rtype(result = "Result<Room>")]
pub struct CreateRoom {
    pub creator_id: i32,
    pub name: String,
    pub avatar: Option<String>,
    pub participants: Option<Vec<i32>>
}

#[derive(Message)]
#[rtype(result = "Result<User>")]
pub struct AddContact {
    pub user_id: i32,
    pub contact_id: i32
}


