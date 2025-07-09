use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use common::{
    id::{ShortId, short_id},
    types::{
        room::{RoomId, RoomScopeType},
        session::{SessionClaim, UserRoomType},
    },
};
use std::time::{Duration, Instant};

use crate::websocket::models::lobby::{Disconnect, Lobby};

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub struct WsConnection {
    pub id: ShortId,
    pub session: SessionClaim,
    pub role: UserRoomType,
    pub room: RoomId,
    pub lobby: Addr<Lobby>,
    pub last_heartbeat: Instant,
}

impl WsConnection {
    pub fn new(
        session: SessionClaim,
        room: RoomId,
        role: UserRoomType,
        lobby: Addr<Lobby>,
    ) -> Self {
        Self {
            id: short_id(Some(10)),
            session,
            room,
            role,
            lobby,
            last_heartbeat: Instant::now(),
        }
    }

    pub fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.last_heartbeat) > CLIENT_TIMEOUT {
                println!("Client timed out");
                ctx.stop();
                return;
            }

            ctx.ping(b"ping");
        });
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.lobby.do_send(Disconnect {
            id: self.id.clone(),
            room: self.room.clone(),
        });
    }
}
