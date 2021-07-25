use std::time::{Duration, Instant};
use actix::{Actor, Addr, WrapFuture, ContextFutureSpawner, StreamHandler, Handler, AsyncContext, ActorContext, Running, ActorFuture};
use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};


use super::state::State;
use crate::messages::ws_messages::{Disconnect, Connect, WsMessage};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocket {
    id: i32,
    lobby: Addr<State>,
    hb: Instant,
}

impl WebSocket {
    pub fn new(lobby: Addr<State>, id: i32) -> Self {
        WebSocket {
            id,
            hb: Instant::now(),
            lobby,
        }
    }
    fn heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx | {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                actor.lobby.do_send(Disconnect {id: actor.id});
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
        let client = ctx.address();
        self.lobby.send(Connect {
            client,
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
        self.lobby.do_send(Disconnect {id: self.id});
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
            Ok(Message::Binary(bin)) => {
                ctx.binary(bin);
            },
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            Ok(Message::Nop) => (),
            Ok(Message::Continuation(_)) => ctx.stop(),
            Ok(Message::Text(_)) => (),
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
