use parking_lot::RwLock;
use actix::{Addr, Actor, Context, Handler, AsyncContext, WrapFuture};
use std::iter::FromIterator;
use std::collections::{HashMap};
use std::sync::Arc;
use futures::StreamExt;

use crate::messages::ws_messages::{RoomMessage, WsMessage, ConnectToRoom, DisconnectFromRoom};
use crate::actors::web_socket::{WebSocket};

pub struct Room {
    pub(crate) participants: Arc<RwLock<HashMap<i32, Addr<WebSocket>>>>,
}

impl Room {
    pub fn new(socket: Addr<WebSocket>, id: i32) -> Addr<Self> {
        let hash_map = HashMap::from_iter(vec![(id, socket)].into_iter());
        let participants = Arc::new(RwLock::new(hash_map));
        Room {
            participants
        }.start()
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Handler<RoomMessage> for Room {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, ctx: &mut Self::Context) -> Self::Result {
        let participants = Arc::clone(&self.participants);
        ctx.spawn(
            async move {
                futures::stream::iter(participants
                    .read()
                    .iter()
                    .map(move |(_, socket)| (socket, msg.msg.clone())))
                    .for_each_concurrent(None, move |(socket, msg)| async move {
                        if let Ok(()) = socket.send(WsMessage(msg)).await {}
                    }).await
            }.into_actor(self)
        );
    }
}

impl Handler<ConnectToRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: ConnectToRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.participants.write().insert(msg.sender_id, msg.addr);
    }
}

impl Handler<DisconnectFromRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.participants.write().remove(&msg.socket_id);
    }
}