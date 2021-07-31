use async_graphql::{Object, Context, Result, Upload, UploadValue};
use actix::{Addr};

use crate::actors::file_writer::{FileWriter};
use crate::messages::db_messages::{SignUp, SignIn, Logout, CreateRoom, AddContact};
use crate::messages::fs_messages::{UserAvatar, RoomAvatar};
use crate::messages::ws_messages::{CreateRoom as CreateRoomSession, AddContactToUser};
use crate::graphql::inputs::{Credentials, RoomInfo};
use crate::graphql::models::user::User;
use crate::actors::mutation_handlers::sign_up_handler::SignUpHandler;
use crate::actors::mutation_handlers::sign_in_handler::SignInHandler;
use crate::actors::mutation_handlers::logout_handler::LogoutHandler;
use crate::graphql::models::room::Room;
use crate::actors::mutation_handlers::room_creator_handler::RoomCreatorHandler;
use crate::actors::state::State;
use crate::actors::mutation_handlers::add_contact_handler::AddContactHandler;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_up(&self, ctx: &Context<'_>, credentials: Credentials) -> anyhow::Result<User> {
        let storage = ctx.data_unchecked::<Addr<SignUpHandler>>();
        let user: anyhow::Result<User> = storage.send(SignUp { name: credentials.name, password: credentials.password }).await?;
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
    async fn load_avatar(&self, ctx: &Context<'_>, user_id: i32, file: Upload) -> Result<&str> {
        let file_info = file.value(ctx)?;
        let file_server = ctx.data_unchecked::<Addr<FileWriter>>();
        file_server.send(UserAvatar {
            user_id,
            content: file_info.content,
            file_name: file_info.filename,
        }).await?;
        Ok("")
    }
    async fn create_room(&self, ctx: &Context<'_>, room_info: RoomInfo) -> anyhow::Result<Room> {
        let mut path: Option<String> = None;
        if let Some(avatar) = room_info.avatar {
            let file_writer = ctx.data_unchecked::<Addr<FileWriter>>();
            let UploadValue { content, filename, .. } = avatar.value(ctx)?;
            let result = file_writer.send(RoomAvatar {
                content,
                file_name: filename,
                room_name: room_info.name.clone(),
            }).await??;
            path = Some(result);
        }
        let room_creator = ctx.data_unchecked::<Addr<RoomCreatorHandler>>();
        let room = room_creator.send(CreateRoom {
            creator_id: room_info.creator_id,
            name: "".to_string(),
            avatar: path,
            participants: room_info.participants,
        }).await??;
        let state = ctx.data_unchecked::<Addr<State>>();
        state.send(CreateRoomSession {
            id: room.id,
            creator_id: room_info.creator_id
        }).await?;
        Ok(room)
    }

    async fn add_contact(&self, ctx: &Context<'_>, user_id: i32, contact_id: i32) -> anyhow::Result<User> {
        let storage = ctx.data_unchecked::<Addr<AddContactHandler>>();
        let user = storage.send(AddContact {
            user_id,
            contact_id
        }).await??;
        let state = ctx.data_unchecked::<Addr<State>>();
        state.send(AddContactToUser {
            user_id,
            contact_id
        }).await?;
        Ok(user)
    }
}