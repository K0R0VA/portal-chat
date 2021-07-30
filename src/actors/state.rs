use crate::actors::room::Room;
use crate::messages::ws_messages::{CreateRoom, Connect, Disconnect, RoomIsEmpty, GetUser, NewRoom};
use crate::actors::user::User;
use crate::future_spawn_ext::FutureSpawnExt;


use actix::{Actor, Context, Handler, Addr, AsyncContext};
use std::collections::{HashMap};

pub struct State {
    rooms: HashMap<i32, Addr<Room>>,
    users: HashMap<i32, Addr<User>>,
}

impl Actor for State {
    type Context = Context<Self>;
}

impl Default for State {
    fn default() -> Self {
        Self {
            rooms: Default::default(),
            users: Default::default(),
        }
    }
}

impl Handler<CreateRoom> for State {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, ctx: &mut Self::Context) -> Self::Result {
        let room = Room::new(msg.id, ctx.address());
        self.users.get(&msg.creator_id)
            .map(|creator| {
                creator.send(NewRoom {
                    id: msg.id,
                    room: room.clone(),
                }).spawn(self, ctx);
            });
        self.rooms.insert(msg.id, room);
    }
}

impl Handler<Connect> for State {
    type Result = Addr<User>;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let state = ctx.address();
        let user_rooms = msg.rooms.iter().map(|key| {
            if let Some((key, room)) = self.rooms.get_key_value(key) {
                return (*key, room.clone());
            }
            let room = self.rooms.insert(*key, Room::new(*key, state.clone()));
            (*key, room.expect(""))
        }).collect();
        let user_friends = self.users.iter()
            .filter(|(id, _)| msg.friends.contains(id))
            .map(|(key, friend)| (*key, friend.clone()))
            .collect();
        User {
            id: msg.user_id,
            sessions: HashMap::with_capacity(1),
            rooms: user_rooms,
            contacts: user_friends,
            state,
        }.start()
    }
}

impl Handler<Disconnect> for State {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.id);
    }
}

impl Handler<RoomIsEmpty> for State {
    type Result = ();

    fn handle(&mut self, msg: RoomIsEmpty, _: &mut Self::Context) -> Self::Result {
        self.rooms.remove(&msg.0);
    }
}

impl Handler<GetUser> for State {
    type Result = Option<Addr<User>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.user_id).map(|user| user.clone())
    }
}
