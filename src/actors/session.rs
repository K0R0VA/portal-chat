use std::time::{Duration, Instant};
use actix::{Actor, Addr, WrapFuture, ContextFutureSpawner, StreamHandler, Handler, AsyncContext, ActorContext, Running, ActorFuture};
use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};


use crate::messages::ws_messages::{WsMessage, NewSession, CloseSession, RoomMessage, PrivateMessage};
use uuid::Uuid;
use crate::actors::user::User;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Session {
    id: Uuid,
    user: Addr<User>,
    hb: Instant,
}

impl Session {
    pub fn new(user: Addr<User>, id: Uuid) -> Self {
        Session {
            id,
            hb: Instant::now(),
            user,
        }
    }
    fn heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx | {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                actor.user.do_send(CloseSession (actor.id));
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut WebsocketContext<Self>) {
        self.heartbeat(ctx);
        let session = ctx.address();
        self.user.send(NewSession {
            session_id: self.id,
            session
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
        let _ = self.user.send(CloseSession(self.id));
        Running::Stop
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Session {
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

impl Handler<WsMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut WebsocketContext<Self>) -> Self::Result {
        ctx.text(msg.0)
    }
}

impl Handler<RoomMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<PrivateMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
