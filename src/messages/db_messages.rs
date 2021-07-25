use actix::Message;
use crate::graphql::models::user::User;
use anyhow::Result;

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


