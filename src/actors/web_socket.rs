use std::time::{Duration, Instant};

use actix::{Actor, Addr, WrapFuture, ActorFutureExt, ContextFutureSpawner, StreamHandler, Handler, AsyncContext, ActorContext, Running};
use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};
use uuid::Uuid;

use super::lobby::Lobby;
use crate::messages::ws_messages::{Disconnect, Socket, Connect, RoomMessage, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocket {
    id: Uuid,
    lobby: Addr<Lobby>,
    hb: Instant,
    room: Uuid,
}

impl WebSocket {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> Self {
        WebSocket {
            id: Uuid::new_v4(),
            hb: Instant::now(),
            room,
            lobby,
        }
    }
    fn heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx | {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                actor.lobby.do_send(Disconnect {id: actor.id, room_id: actor.room});
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for WebSocket {
    type Context = WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut WebsocketContext<Self>) {
        self.heartbeat(ctx);
        let addr = ctx.address();
        let socket: Socket = addr.recipient::<WsMessage>();
        self.lobby.send(Connect {
            addr: socket,
            lobby_id: self.room,
            user_id: self.id
        })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            }).wait(ctx);
    }
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.lobby.do_send(Disconnect {id: self.id, room_id: self.room});
        Running::Stop
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for WebSocket {
    fn handle(&mut self, message: Result<Message, ProtocolError>, ctx: &mut WebsocketContext<Self>) {
        match message {
            Ok(Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            Ok(Message::Nop) => (),
            Ok(Message::Continuation(_)) => ctx.stop(),
            Ok(Message::Text(msg)) => self.lobby.do_send(RoomMessage {
                sender_id: self.id,
                msg: msg.to_string(),
                room_id: self.room
            }),
            Err(_e) => panic!()
        }
    }
}

impl Handler<WsMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut WebsocketContext<Self>) -> Self::Result {
        ctx.text(msg.0)
    }
}
