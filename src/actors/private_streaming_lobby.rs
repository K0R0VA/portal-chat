// use crate::actors::streaming_session::{StreamingSession, StreamSessionStatus};
// use actix::{Addr, Actor, Context, Handler};
// use uuid::Uuid;
// use crate::messages::stream_messages::{LeavePrivateLobby, ConnectToLobby, ConsumerLeave};

// pub struct PrivateStreamingLobby {
//     id: Uuid,
//     caller: Option<Addr<StreamingSession>>,
//     receiver: Option<Addr<StreamingSession>>
// }

// impl Actor for PrivateStreamingLobby {
//     type Context = Context<Self>;
// }

// impl Handler<LeavePrivateLobby> for PrivateStreamingLobby {
//     type Result = ();

//     fn handle(&mut self, msg: LeavePrivateLobby, ctx: &mut Self::Context) -> Self::Result {
//         match msg.session_status {
//             StreamSessionStatus::Caller => {
//                 self.caller = None;
//                 if let Some(receiver) = self.receiver.as_ref() {
//                     receiver.send(ConsumerLeave);
//                 }
//             }
//             StreamSessionStatus::Receiver => {

//             }
//         }
//     }
// }

// impl Handler<ConnectToLobby> for PrivateStreamingLobby {
//     type Result = ();

//     fn handle(&mut self, msg: ConnectToLobby, ctx: &mut Self::Context) -> Self::Result {
//         match msg.session_status {
//             StreamSessionStatus::Caller => self.caller = Some(msg.session),
//             StreamSessionStatus::Receiver => self.receiver = Some(msg.session)
//         }
//     }
// }