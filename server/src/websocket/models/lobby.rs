use actix::{Actor, Addr, Context, Message};
use common::{id::ShortId, types::room::RoomId};
use std::collections::HashMap;

use crate::websocket::models::connection::WsConnection;

#[derive(Clone, Debug)]
pub struct Lobby {
    rooms: HashMap<RoomId, HashMap<ShortId, (Addr<WsConnection>, String)>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: ShortId,
    pub name: String,
    pub room: String,
    pub addr: Addr<WsConnection>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: ShortId,
    pub room: String,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
