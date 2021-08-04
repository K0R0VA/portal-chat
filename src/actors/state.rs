use crate::actors::room::Room;
use crate::messages::ws_messages::{CreateRoom, Connect, Disconnect, RoomIsEmpty, GetUser, NewRoom, AddContactToUser, AddContactActor, GetUserChats, UserChats, UserChatsIds};
use crate::actors::user::User;
use crate::extensions::future_spawn_ext::FutureSpawnExt;
use crate::actors::state_storage::StateStorage;
use crate::actors::session::Session;

use actix::{Actor, Context, Handler, Addr, AsyncContext, ResponseFuture, WrapFuture, ActorFuture, ContextFutureSpawner};
use std::collections::{HashMap};
use std::sync::Arc;
use uuid::Uuid;
use async_graphql::dataloader::DataLoader;
use deadpool_postgres::Pool;
use async_graphql::futures_util::FutureExt;

pub struct State {
    rooms: HashMap<i32, Addr<Room>>,
    users: HashMap<i32, Addr<User>>,
    storage: Arc<DataLoader<StateStorage>>,
}

impl Actor for State {
    type Context = Context<Self>;
}

impl State {
    pub fn new(pool: Pool) -> Self {
        Self {
            rooms: Default::default(),
            users: Default::default(),
            storage: Arc::new(DataLoader::new(StateStorage { pool })),
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
    type Result = Session;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let user = self.users
            .entry(msg.user_id)
            .or_insert(
                User {
                    id: msg.user_id,
                    sessions: Default::default(),
                    rooms: Default::default(),
                    contacts: Default::default(),
                    state: ctx.address(),
                }.start()
            );
        Session::new(user.clone(), Uuid::new_v4())
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
        self.users
            .get(&msg.user_id)
            .map(|user| user.clone())
    }
}

impl Handler<AddContactToUser> for State {
    type Result = ();

    fn handle(&mut self, msg: AddContactToUser, ctx: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.user_id).map(|user| {
            user.send(AddContactActor {
                user_id: 0
            }).spawn(self, ctx);
        });
    }
}
impl Handler<GetUserChats> for State {
    type Result = ();

    fn handle(&mut self, msg: GetUserChats, ctx: &mut Self::Context) -> Self::Result {
        // let storage= Arc::clone(&self.storage);
        // Box::pin(async move {
        //     storage
        //         .load_one(msg.user_id).await
        // })
        //     .into_actor(self)
        //     .then(|res, actor, ctx| {
        //         if let Ok(Some(UserChatsIds { contacts, rooms })) = res {
        //             let contacts = contacts
        //                 .iter()
        //                 .filter_map(|id| self.users
        //                     .get_key_value(id)
        //                     .map(|(key, contact)| (*key, contact.clone())))
        //                 .collect();
        //             let rooms = rooms
        //                 .iter()
        //                 .map(|key| {
        //                     (
        //                         *key,
        //                         self.rooms
        //                             .entry(*key)
        //                             .or_insert(Room {
        //                                 id: *key,
        //                                 participants: Default::default(),
        //                                 state: ctx.address(),
        //                             }.start()).clone()
        //                     )
        //                 })
        //                 .collect();
        //             msg.user_address.send(UserChats { contacts, rooms }).spawn(actor, ctx);
        //         }
        //         actix::fut::ready(())
        //     })
        //     .wait(ctx);
    }
}
