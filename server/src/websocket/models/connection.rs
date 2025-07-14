use crate::websocket::models::lobby::{Connect, Disconnect, Lobby};
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, fut::wrap_future};
use actix_web_actors::ws;
use common::types::{
    room::RoomId,
    session::{SessionClaim, UserRoomType},
};
use std::time::{Duration, Instant};

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub struct WsConnection {
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

       let fut = self.lobby.send(Connect {
            room: self.room.clone(),
            connection: self.session.clone(),
            addr: ctx.address(),
            role: self.role.clone(),
        });

        ctx.spawn(wrap_future(fut).map(|_res, _actor, _ctx| ()));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let fut = self.lobby.send(Disconnect {
            room: self.room.clone(),
            session: self.session.clone(),
        });

        ctx.spawn(wrap_future(fut).map(|_res, _actor, _ctx| ()));
    }
}
