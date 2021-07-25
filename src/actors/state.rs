use crate::actors::room::Room;
use crate::messages::ws_messages::{ CreateRoom, Connect, Disconnect, RoomMessage, PrivateMessage, WsMessage};
use crate::actors::web_socket::{WebSocket};

use actix::{Actor, Context, Handler, Addr};
use std::collections::{HashMap};

pub struct State {
    rooms: HashMap<i32, Addr<Room>>,
    sessions: HashMap<i32, Addr<WebSocket>>
}

impl Actor for State {
    type Context = Context<Self>;
}

impl Default for State {
    fn default() -> Self {
       State {
           rooms: HashMap::new(),
           sessions: HashMap::new()
       }
    }
}

impl Handler<CreateRoom> for State {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, _ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.get(&msg.creator_id) {
            Some(socket) => {
                self.rooms.insert(msg.id, Room::new(socket.clone(), msg.creator_id)).unwrap();
            },
            None => {}
        }
    }
}

impl Handler<Connect> for State {
    type Result = ();
    fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {

    }
}

impl Handler<Disconnect> for State {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
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

impl Handler<RoomMessage> for State {
    type Result = ();
    fn handle(&mut self, msg: RoomMessage, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get(&msg.room_id) {
            room.do_send(msg);
        }
    }
}

impl Handler<PrivateMessage> for State {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(recipient) = self.sessions.get(&msg.recipient_id) {
            recipient.do_send(WsMessage(msg.msg.clone()));
        }
        if let Some(()) = self.sessions.get(&msg.id).and_then(|sender| Some(sender.do_send(WsMessage(msg.msg)))) {}
    }
}
