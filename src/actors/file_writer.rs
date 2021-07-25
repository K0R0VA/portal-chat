use actix::{Actor, Context, Handler, AsyncContext, WrapFuture};
use tokio::io::AsyncWriteExt;
use std::io::Read;

use crate::messages::fs_messages::Avatar;

pub struct FileWriter;

impl Actor for FileWriter {
    type Context = Context<Self>;
}

impl Handler<Avatar> for FileWriter {
    type Result = ();

    fn handle(&mut self, msg: Avatar, ctx: &mut Self::Context) -> Self::Result {
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

