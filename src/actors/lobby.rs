use actix::{Actor, Context, Handler, Addr};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::actors::room::Room;
use actix::dev::MessageResponse;
use std::sync::Arc;
use parking_lot::lock_api::RwLock;
use crate::messages::ws_messages::{Socket, CreateRoom, Connect, Disconnect, RoomMessage, PrivateMessage, WsMessage};

pub struct Lobby {
    rooms: HashMap<Uuid, Addr<Room>>,
    sessions: HashMap<Uuid, Arc<Socket>>
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Default for Lobby {
    fn default() -> Self {
       Lobby {
           rooms: HashMap::new(),
           sessions: HashMap::new()
       }
    }
}

impl Handler<CreateRoom> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.get(&msg.creator_id) {
            Some(socket) => {
                self.rooms.insert(msg.id, Room::new(Arc::clone(socket))).unwrap();
            },
            None => {}
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {

    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        // if let Some(_) = self.sessions.remove(&msg.id) {
        //     self.rooms
        //         .get(&msg.room_id)
        //         .unwrap()
        //         .iter()
        //         .filter(|conn_id| *conn_id.to_owned() != msg.id)
        //         .for_each(|conn_id| {
        //             self.send_message(&format!("{} покинул беседу", &msg.id), conn_id)
        //         });
        //     if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
        //         if lobby.is_empty() {
        //             self.rooms.remove(&msg.room_id);
        //         } else {
        //             lobby.remove(&msg.id);
        //         }
        //     }
        // }
    }
}

impl Handler<RoomMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: RoomMessage, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get(&msg.room_id) {
            room.do_send(msg);
        }
    }
}

impl Handler<PrivateMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, ctx: &mut Self::Context) -> Self::Result {
        if let Some(recipient) = self.sessions.get(&msg.recipient_id) {
            recipient.do_send(WsMessage(msg.msg.clone()));
        }
        self.sessions.get(&msg.id).and_then(|sender| Some(sender.do_send(WsMessage(msg.msg)))).unwrap();
    }
}
