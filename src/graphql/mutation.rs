use async_graphql::{Object, Context, Result, Upload};
use actix::Addr;

use crate::actors::file_writer::{FileWriter};
use crate::messages::db_messages::{SignUp, SignIn, Logout};
use crate::messages::fs_messages::Avatar;
use crate::graphql::inputs::Credentials;
use crate::graphql::models::user::User;
use crate::actors::mutation_handlers::sign_up_handler::SignUpHandler;
use crate::actors::mutation_handlers::sign_in_handler::SignInHandler;
use crate::actors::mutation_handlers::logout_handler::LogoutHandler;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_up(&self, ctx: &Context<'_>, credentials: Credentials) -> anyhow::Result<User> {
        let storage = ctx.data_unchecked::<Addr<SignUpHandler>>();
        let user: anyhow::Result<User>  = storage.send(SignUp { name: credentials.name, password: credentials.password }).await?;
        user
    }
    async fn sign_id(&self, ctx: &Context<'_>, credentials: Credentials) -> anyhow::Result<User> {
        let storage = ctx.data_unchecked::<Addr<SignInHandler>>();
        let user = storage.send(SignIn { name: credentials.name, password: credentials.password }).await?;
        user
    }

    async fn logout(&self, ctx: &Context<'_>, user_id: i32) -> anyhow::Result<i32> {
        let storage = ctx.data_unchecked::<Addr<LogoutHandler>>();
        storage.send(Logout { user_id }).await??;
        Ok(user_id)
    }

    async fn load_avatar(&self, ctx: &Context<'_>, user_id: i32, file: Upload) -> Result<bool> {
        let file_info = file.value(ctx)?;
        let file_server = ctx.data_unchecked::<Addr<FileWriter>>();
        file_server.send(Avatar {
            user_id,
            content: file_info.content,
            file_name: file_info.filename
        }).await?;
        Ok(true)
    }
}