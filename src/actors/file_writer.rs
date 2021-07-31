use actix::{Actor, Context, Handler, AsyncContext, WrapFuture, ResponseFuture};
use tokio::io::AsyncWriteExt;
use std::io::Read;

use crate::messages::fs_messages::{UserAvatar, RoomAvatar};

pub struct FileWriter;

impl Actor for FileWriter {
    type Context = Context<Self>;
}

impl Handler<UserAvatar> for FileWriter {
    type Result = ();

    fn handle(&mut self, msg: UserAvatar, ctx: &mut Self::Context) -> Self::Result {
        use tokio::fs;
        ctx.spawn(async move {
            let result: anyhow::Result<()> = try {
                fs::create_dir(format!("sources/{}", msg.user_id)).await?;
                let mut file = fs::File::create(format!("sources/{}", msg.to_string())).await?;
                file.write_all(&msg.content.bytes().map(|byte|
                    byte.unwrap())
                    .collect::<Vec<u8>>()).await?;
            };
            result.unwrap()
        }.into_actor(self));
    }
}

impl Handler<RoomAvatar> for FileWriter {
    type Result = ResponseFuture<anyhow::Result<String>>;

    fn handle(&mut self, msg: RoomAvatar, _: &mut Self::Context) -> Self::Result {
        use tokio::fs;
        Box::pin(async move {
            try {
                let _ = fs::create_dir_all(format!("sources/{}/avatar", msg.room_name)).await;
                let relative_path = msg.to_string();
                let path = format!("sources/{relative_path}");
                let mut file = fs::File::create(&path).await?;
                file.write_all(&msg.content.bytes().map(|byte|
                    byte.unwrap())
                    .collect::<Vec<u8>>()).await?;
                path
            }
        })
    }
}


