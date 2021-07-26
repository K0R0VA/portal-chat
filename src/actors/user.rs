use std::collections::HashMap;
use uuid::Uuid;
use actix::{Addr, Actor, Context, Handler, ActorContext, Running, WrapFuture, ActorFuture, ContextFutureSpawner};

use crate::actors::session::Session;
use crate::actors::room::Room;
use crate::actors::state::State;
use crate::messages::ws_messages::{NewSession, ConnectToRoom, CloseSession, DisconnectFromRoom, Disconnect, PrivateMessage, PrivateMessageToContact, AddContact, GetUser, NewRoom};

pub struct User {
    pub id: i32,
    pub sessions: HashMap<Uuid, Addr<Session>>,
    pub rooms: HashMap<i32, Addr<Room>>,
    pub contacts: HashMap<i32, Addr<User>>,
    pub state: Addr<State>
}


impl Actor for User {
    type Context = Context<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let _ = self.state.send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<NewSession> for User {
    type Result = ();

    fn handle(&mut self, msg: NewSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.session_id, msg.session.clone());
        self.rooms.iter().for_each(|(_, room)| {
            let _ = room.send(ConnectToRoom { session: msg.session.clone(), session_id: msg.session_id });
        })
    }
}

impl Handler<CloseSession> for User {
    type Result = ();

    fn handle(&mut self, msg: CloseSession, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.0);
        self.rooms.iter().for_each(|(_, room)| {
            let _ = room.send(DisconnectFromRoom { socket_id: msg.0 });
        });
        self.sessions
            .is_empty()
            .then(|| ctx.stop());
    }
}

impl Handler<PrivateMessageToContact> for User {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessageToContact, _: &mut Self::Context) -> Self::Result {
        self.sessions.iter().for_each(|(_, session)| {
            let _ = session.send(PrivateMessage {
                user_id: self.id,
                friend_id: msg.friend_id,
                msg: msg.msg.clone()
            });
        });
        if let Some(recipient) = self.contacts.get(&msg.friend_id) {
            let _ = recipient.send(PrivateMessage {
                user_id: self.id,
                friend_id: msg.friend_id,
                msg: msg.msg
            });
        }
    }
}

impl Handler<AddContact> for User {
    type Result = ();

    fn handle(&mut self, msg: AddContact, ctx: &mut Self::Context) -> Self::Result {
        let contact_id = msg.user_id;
        self.state.send(GetUser{ user_id: contact_id })
            .into_actor(self)
            .then(move |res, actor, _| {
                if let Ok(Some(user)) = res {
                    actor.contacts.insert(contact_id, user);
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<PrivateMessage> for User {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, _: &mut Self::Context) -> Self::Result {
        self.sessions.iter().for_each(|(_, session)| {
            let _ = session.send(msg.clone());
        });
    }
}

impl Handler<NewRoom> for User {
    type Result = ();

    fn handle(&mut self, msg: NewRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.insert(msg.id, msg.room);
    }
}