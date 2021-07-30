use std::fs::File;
use actix::Message;

#[derive(Message)]
#[rtype(Result = "()")]
pub struct UserAvatar {
    pub content: File,
    pub user_id: i32,
    pub file_name: String,
}

impl ToString for UserAvatar {
    fn to_string(&self) -> String {
        format!("users/{}/avatar/{}", self.user_id, &self.file_name)
    }
}

#[derive(Message)]
#[rtype(Result = "anyhow::Result<String>")]
pub struct RoomAvatar {
    pub content: File,
    pub room_id: i32,
    pub file_name: String,
}


impl ToString for RoomAvatar {
    fn to_string(&self) -> String {
        format!("rooms/{}/avatar/{}", self.room_id, &self.file_name)
    }
}


