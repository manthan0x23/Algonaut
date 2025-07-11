use actix::{ActorContext, Handler, StreamHandler};
use actix_web_actors::ws;
use std::time::Instant;

use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    lobby::{Connect, Disconnect, Lobby},
};

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let room = self.rooms.entry(msg.room).or_default();

        room.insert(msg.connection.uid, msg.addr.clone());

        let broadcast = Broadcast {
            message: format!(
                "{} joined the room",
                msg.connection
                    .user
                    .name
                    .clone()
                    .or(Some(msg.connection.user.email.clone()))
                    .unwrap_or_else(|| "Unknown user".to_string())
            ),
        };

        let json =
            serde_json::to_string(&broadcast).unwrap_or("False broadcast loading".to_string());

        for (_, ws_conn) in room {
            ws_conn.do_send(WsText(json.clone()));
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text = text.to_string();

                println!("text :: {}", text);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {
                ctx.text("error: binary not supported");
            }
            _ => {}
        }
    }
}

impl Handler<WsText> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsText, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}
