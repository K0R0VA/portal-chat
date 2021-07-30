use std::time::{Duration, Instant};
use actix::{Actor, Addr, WrapFuture, ContextFutureSpawner, StreamHandler, Handler, AsyncContext, ActorContext, Running, ActorFuture};
use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};
use uuid::Uuid;


use crate::actors::user::User;
use crate::messages::ws_messages::{NewSession, CloseSession, RoomMessage, PrivateMessage, PrivateMessageToContact};
use actix_web::client::WsProtocolError;

use crate::proto::{deserialize_client_message, message::MessageType, serialize_server_message};
use crate::proto::message::ServerMessage;
use crate::future_spawn_ext::FutureSpawnExt;

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
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                actor.user.do_send(CloseSession(actor.id));
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
            session,
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

impl StreamHandler<Result<Message, WsProtocolError>> for Session {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Binary(ref bytes)) => {
                let message = deserialize_client_message(bytes);
                match MessageType::from_i32(message.request_type) {
                    Some(MessageType::Contact) => {
                        self.user.send(PrivateMessageToContact {
                            contact_id: message.recipient_id,
                            msg: message.text,
                        }).spawn(self, ctx);
                    }
                    Some(MessageType::Group) => {
                        self.user.send(RoomMessage {
                            room_id: message.recipient_id,
                            sender_id: message.sender_id,
                            msg: message.text,
                        }).spawn(self, ctx);
                    }
                    _ => unimplemented!()
                }
            }
            Ok(Message::Ping(ref msg)) => {
                self.hb = Instant::now();
                ctx.pong(msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => unimplemented!()
        }
    }
}

impl Handler<RoomMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, ctx: &mut Self::Context) -> Self::Result {
        let server_message = ServerMessage {
            sender_id: msg.sender_id,
            text: msg.msg,
            request_type: MessageType::Group as i32,
        };
        let bytes = serialize_server_message(server_message);
        ctx.binary(bytes);
    }
}

impl Handler<PrivateMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, ctx: &mut Self::Context) -> Self::Result {
        let server_message = ServerMessage {
            sender_id: msg.contact_id,
            text: msg.msg,
            request_type: MessageType::Contact as i32,
        };
        let bytes = serialize_server_message(server_message);
        ctx.binary(bytes);
    }
}
