use std::fs::File;
use actix::Message;

#[derive(Message)]
#[rtype(Result = "()")]
pub struct Avatar {
    pub content: File,
    pub user_id: i32,
    pub file_name: String,
}

impl ToString for Avatar {
    fn to_string(&self) -> String {
        format!("{}/{}", self.user_id, &self.file_name)
    }
}
