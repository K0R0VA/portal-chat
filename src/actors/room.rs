use actix::{Addr, Actor, Context, Handler, ActorContext, Running};
use std::collections::{HashMap};
use uuid::Uuid;


use crate::messages::ws_messages::{RoomMessage, ConnectToRoom, DisconnectFromRoom, RoomIsEmpty};
use crate::actors::session::{Session};
use crate::actors::state::State;

pub struct Room {
    pub id: i32,
    pub participants: HashMap<Uuid, Addr<Session>>,
    pub state: Addr<State>
}

impl Room {
    pub fn new(id: i32, state: Addr<State>) -> Addr<Self> {
        Self {
            id,
            participants: Default::default(),
            state
        }.start()
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        let _ = self.state.send(RoomIsEmpty(self.id));
        Running::Stop
    }
}

impl Handler<RoomMessage> for Room {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, _: &mut Self::Context) -> Self::Result {
        self.participants.iter().for_each(|(_, participant)| {
            let _ = participant.send(msg.clone());
        })
    }
}

impl Handler<ConnectToRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: ConnectToRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.participants.insert(msg.session_id, msg.session);
    }
}

impl Handler<DisconnectFromRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromRoom, ctx: &mut Self::Context) -> Self::Result {
        self.participants.remove(&msg.socket_id);
        self.participants.is_empty().then(|| ctx.stop());
    }
}