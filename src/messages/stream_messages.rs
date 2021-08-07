use crate::actors::streaming_session::{StreamSessionStatus, StreamingSession};

use actix::{Message, Addr};
use mediasoup::consumer::ConsumerId;

#[derive(Message)]
#[rtype(Result = "()")]
pub struct LeavePrivateLobby {
    pub session_status: StreamSessionStatus
}

#[derive(Message)]
#[rtype(Result = "()")]
pub struct ConsumerLeave {
    id: ConsumerId
}

#[derive(Message)]
#[rtype(Result = "()")]
pub struct ConnectToLobby {
    pub session_status: StreamSessionStatus,
    pub session: Addr<StreamingSession>
}