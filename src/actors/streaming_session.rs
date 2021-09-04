// use std::num::{NonZeroU32, NonZeroU8};
// use mediasoup::prelude::*;
// use std::collections::HashMap;
// use actix::{Actor, Running, Addr, ActorFuture, AsyncContext, StreamHandler, ActorContext, Handler};
// use actix_web_actors::ws::{WebsocketContext, Message, ProtocolError};

// use crate::actors::private_streaming_lobby::PrivateStreamingLobby;
// use crate::messages::stream_messages::{ConnectToLobby, LeavePrivateLobby, ConsumerLeave};
// use crate::extensions::future_spawn_ext::FutureSpawnExt;
// use crate::actors::Session;
// use std::time::{Duration, Instant};


// fn media_codecs() -> [RtpCodecCapability; 2] {
//     [
//         RtpCodecCapability::Audio {
//             mime_type: MimeTypeAudio::Opus,
//             preferred_payload_type: None,
//             clock_rate: NonZeroU32::new(48000).unwrap(),
//             channels: NonZeroU8::new(2).unwrap(),
//             parameters: RtpCodecParametersParameters::from([("useinbandfec", 1_u32.into())]),
//             rtcp_feedback: vec![RtcpFeedback::TransportCc],
//         },
//         RtpCodecCapability::Video {
//             mime_type: MimeTypeVideo::Vp9,
//             preferred_payload_type: None,
//             clock_rate: NonZeroU32::new(90000).unwrap(),
//             parameters: RtpCodecParametersParameters::default(),
//             rtcp_feedback: vec![
//                 RtcpFeedback::Nack,
//                 RtcpFeedback::NackPli,
//                 RtcpFeedback::CcmFir,
//                 RtcpFeedback::GoogRemb,
//                 RtcpFeedback::TransportCc,
//             ],
//         },
//     ]
// }

// struct Transports {
//     consumer: WebRtcTransport,
//     producer: WebRtcTransport,
// }

// #[repr(u8)]
// #[derive(Copy, Clone)]
// pub enum StreamSessionStatus {
//     Caller,
//     Receiver
// }

// pub struct StreamingSession {
//     client_rtp_capabilities: Option<RtpCapabilities>,
//     consumers: HashMap<ConsumerId, Consumer>,
//     producer: Option<Producer>,
//     router: Router,
//     transports: Transports,
//     lobby: Addr<PrivateStreamingLobby>,
//     status: StreamSessionStatus,
//     hb: Instant
// }

// impl StreamingSession {
//     async fn new (worker_manager: &WorkerManager, lobby: Addr<PrivateStreamingLobby>, status: StreamSessionStatus) -> anyhow::Result<Self> {
//         let worker = worker_manager
//             .create_worker(WorkerSettings::default())
//             .await
//             .map_err(|error| format!("Failed to create worker: {}", error))?;
//         let router = worker
//             .create_router(RouterOptions::new(Vec::from(media_codecs())))
//             .await
//             .map_err(|error| format!("Failed to create router: {:?}", error))?;
//         let transport_options =
//             WebRtcTransportOptions::new(TransportListenIps::new(TransportListenIp {
//                 ip: "192.168.0.7".parse().unwrap(),
//                 announced_ip: None,
//             }));
//         let producer_transport = router
//             .create_webrtc_transport(transport_options.clone())
//             .await
//             .map_err(|error| format!("Failed to create producer transport: {:?}", error))?;
//         Ok(Self {
//             client_rtp_capabilities: None,
//             consumers: HashMap::new(),
//             producer: None,
//             router,
//             transports: Transports {
//                 consumer: consumer_transport,
//                 producer: producer_transport,
//             },
//             lobby,
//             status
//         })
//     }
// }

// impl Session for StreamingSession {
//     const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
//     const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

//     fn get_heartbeat(&self) -> Instant {
//         self.hb
//     }
// }


// impl Actor for StreamingSession {
//     type Context = WebsocketContext<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         let server_init_message = ServerMessage::Init {
//             consumer_transport_options: TransportOptions {
//                 id: self.transports.consumer.id(),
//                 dtls_parameters: self.transports.consumer.dtls_parameters(),
//                 ice_candidates: self.transports.consumer.ice_candidates().clone(),
//                 ice_parameters: self.transports.consumer.ice_parameters().clone(),
//             },
//             producer_transport_options: TransportOptions {
//                 id: self.transports.producer.id(),
//                 dtls_parameters: self.transports.producer.dtls_parameters(),
//                 ice_candidates: self.transports.producer.ice_candidates().clone(),
//                 ice_parameters: self.transports.producer.ice_parameters().clone(),
//             },
//             router_rtp_capabilities: self.router.rtp_capabilities().clone(),
//         };
//         ctx.binary(vec![]);
//         self.lobby.send(ConnectToLobby {
//             session_status: self.status,
//             session: ctx.address().clone()
//         })
//             .spawn(self, ctx);
//     }

//     fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
//         self.lobby.send(LeavePrivateLobby {
//             session_status: self.status
//         }).spawn(self, ctx);
//         Running::Stop
//     }
// }

// impl StreamHandler<Result<Message, ProtocolError>> for StreamingSession {
//     fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(Message::Ping(ref msg)) => ctx.pong(msg),
//             Ok(Message::Pong(_)) => self.hb = Instant::now(),
//             Ok(Message::Close(reason)) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             },
//             Ok(Message::Binary(ref bytes)) => {

//             }
//             Err(_) => {
//                 ctx.stop();
//             },
//             _ => unimplemented!()
//         }
//     }
// }

// impl Handler<ConsumerLeave> for StreamingSession {
//     type Result = ();

//     fn handle(&mut self, msg: ConsumerLeave, ctx: &mut Self::Context) -> Self::Result {

//     }
// }