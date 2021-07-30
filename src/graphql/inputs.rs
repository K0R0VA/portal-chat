use async_graphql::{InputObject, Upload};

#[derive(InputObject)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct RoomInfo {
    pub creator_id: i32,
    pub name: String,
    pub avatar: Option<Upload>
}