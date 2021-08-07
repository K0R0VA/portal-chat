use std::time::{Duration, Instant};
use actix::{Actor, Addr, WrapFuture, ContextFutureSpawner, StreamHandler, Handler, AsyncContext, ActorContext, Running, ActorFuture, Context};
use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};
use uuid::Uuid;


use crate::actors::user::User;
use crate::messages::ws_messages::{NewSession, CloseSession, RoomMessage, PrivateMessage, PrivateMessageToContact, Connect};
use actix_web::client::WsProtocolError;

use crate::proto::{deserialize_client_message, message::MessageType, serialize_server_message};
use crate::proto::message::ServerMessage;
use crate::extensions::future_spawn_ext::FutureSpawnExt;
use actix::dev::{MessageResponse, ResponseChannel};
use crate::actors::state::ChatState;
use crate::actors::Session;

pub struct CommonSession {
    id: Uuid,
    user: Addr<User>,
    hb: Instant,
}

impl MessageResponse<ChatState, Connect> for CommonSession {
    fn handle<R: ResponseChannel<Connect>>(self, ctx: &mut Context<ChatState>, tx: Option<R>) {
        if let Some(sender) = tx {
            sender.send(self)
        }
    }
}


impl CommonSession {
    pub fn new(user: Addr<User>, id: Uuid) -> Self {
        CommonSession {
            id,
            hb: Instant::now(),
            user,
        }
    }
}

impl Session for CommonSession {

    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

    fn get_heartbeat(&self) -> Instant {
        self.hb
    }
}

impl Actor for CommonSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut WebsocketContext<Self>) {
        self.init_heartbeat(ctx);
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

impl StreamHandler<Result<Message, WsProtocolError>> for CommonSession {
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

impl Handler<RoomMessage> for CommonSession {
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

impl Handler<PrivateMessage> for CommonSession {
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
