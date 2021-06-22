use super::ws_message::{ClientMessage, Connect, Disconnect, Socket, WsMessage};
use actix::{Actor, Context, Handler};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Default for Lobby {
    fn default() -> Self {
       Lobby {
           sessions: HashMap::new(),
           rooms: HashMap::new(),
       }
    }
}

impl Lobby {
    fn send_message(&self, msg: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(msg.to_owned()));
            return;
        }
        println!("not found user id");
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);
        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                self.send_message(
                    &format!("{} присоединился к обсуждению", msg.self_id),
                    conn_id,
                )
            });
        self.sessions.insert(msg.self_id, msg.addr);
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id)
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(_) = self.sessions.remove(&msg.id) {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|conn_id| {
                    self.send_message(&format!("{} покинул беседу", &msg.id), conn_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.is_empty() {
                    self.rooms.remove(&msg.room_id);
                } else {
                    lobby.remove(&msg.id);
                }
            }
        }
    }
}

impl Handler<ClientMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&msg.msg, client));
    }
}
