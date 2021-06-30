use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashSet;
use actix::{Addr, Actor, Context, Handler, AsyncContext, WrapFuture};
use std::iter::FromIterator;
use futures::StreamExt;
use crate::messages::ws_messages::{RoomMessage, WsMessage, ConnectToRoom, DisconnectFromRoom, Socket};

pub struct Room {
    pub(crate) participants: Arc<RwLock<HashSet<Arc<Socket>>>>,
}

impl Room {
    pub fn new(socket: Arc<Socket>) -> Addr<Self> {
        let hash_map: HashSet<Arc<Socket>> = HashSet::from_iter(vec![socket].into_iter());
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
        ctx.spawn(async move {
            futures::stream::iter(participants.read().iter()
                .map(|socket| (socket, msg.msg.clone())))
                .for_each(|(socket, msg)| async move {
                    socket.send(WsMessage(msg)).await;
                }).await;
        }.into_actor(self));
    }
}

impl Handler<ConnectToRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: ConnectToRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.participants.write().insert( msg.addr);
    }
}

impl Handler<DisconnectFromRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.participants.write().remove(&*msg.socket);
    }
}