use std::collections::HashMap;
use uuid::Uuid;
use actix::{Addr, Actor, Context, Handler, ActorContext, Running, WrapFuture, ActorFuture, ContextFutureSpawner, AsyncContext};

use crate::actors::session::CommonSession;
use crate::actors::room::Room;
use crate::actors::state::ChatState;
use crate::messages::ws_messages::{NewSession, ConnectToRoom, CloseSession, DisconnectFromRoom, Disconnect, PrivateMessage, PrivateMessageToContact, GetUser, NewRoom, RoomMessage, AddContactActor, GetUserChats, UserChats};
use crate::extensions::future_spawn_ext::{FutureSpawnExt};

pub struct User {
    pub id: i32,
    pub sessions: HashMap<Uuid, Addr<CommonSession>>,
    pub rooms: HashMap<i32, Addr<Room>>,
    pub contacts: HashMap<i32, Addr<User>>,
    pub state: Addr<ChatState>,
}


impl Actor for User {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.state.send(GetUserChats {
            user_address: ctx.address().recipient(),
            user_id: self.id
        }).spawn(self, ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.state.send(Disconnect { id: self.id }).spawn(self, ctx);
        Running::Stop
    }
}

impl Handler<NewSession> for User {
    type Result = ();

    fn handle(&mut self, msg: NewSession, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.session_id, msg.session.clone());
        self.rooms.iter().for_each(|(_, room)| {
            room.send(ConnectToRoom
            {
                session: msg.session.clone(),
                session_id: msg.session_id,
            })
                .spawn(self, ctx);
        })
    }
}

impl Handler<CloseSession> for User {
    type Result = ();

    fn handle(&mut self, msg: CloseSession, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.0);
        self.rooms.iter().for_each(|(_, room)| {
            room.send(DisconnectFromRoom {
                session_id: msg.0
            }).spawn(self, ctx);
        });
        self.sessions
            .is_empty()
            .then(|| ctx.stop());
    }
}

impl Handler<PrivateMessageToContact> for User {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessageToContact, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.iter().for_each(|(_, session)| {
            session.send(PrivateMessage {
                user_id: self.id,
                contact_id: msg.contact_id,
                msg: msg.msg.clone(),
            }).spawn(self, ctx);
        });
        self.contacts.get(&msg.contact_id).and_then(|contact| {
            Some(contact.send(
                PrivateMessage {
                    user_id: self.id,
                    contact_id: msg.contact_id,
                    msg: msg.msg,
                })
                .spawn(self, ctx))
        });
    }
}

impl Handler<AddContactActor> for User {
    type Result = ();

    fn handle(&mut self, msg: AddContactActor, ctx: &mut Self::Context) -> Self::Result {
        let contact_id = msg.user_id;
        self.state.send(GetUser { user_id: contact_id })
            .into_actor(self)
            .then(move |res, actor, _| {
                if let Ok(Some(user)) = res {
                    actor.contacts.insert(contact_id, user);
                }
                actix::fut::ready(())
            })
            .spawn(ctx);
    }
}

impl Handler<PrivateMessage> for User {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.iter().for_each(|(_, session)| {
            session.send(msg.clone()).spawn(self, ctx);
        });
    }
}

impl Handler<NewRoom> for User {
    type Result = ();

    fn handle(&mut self, msg: NewRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.insert(msg.id, msg.room);
    }
}

impl Handler<RoomMessage> for User {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, ctx: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.room_id).and_then(|room| {
            room.send(msg).spawn(self, ctx);
            Some(())
        });
    }
}

impl Handler<UserChats> for User {
    type Result = ();

    fn handle(&mut self, msg: UserChats, ctx: &mut Self::Context) -> Self::Result {
        self.rooms = msg.rooms;
        self.contacts = msg.contacts;
    }
}

