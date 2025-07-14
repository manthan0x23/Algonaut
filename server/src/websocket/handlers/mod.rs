use actix::fut::{ActorFutureExt, wrap_future};
use actix::{ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use common::types::session::UserMinimal;
use std::time::Instant;
use tracing::warn;

use crate::websocket::models::lobby::HandleChat;
use crate::websocket::models::{
    connection::WsConnection, incomming::IncomingMessage, lobby::HandleCrdtUpdate,
};

mod chat;
mod connection;
mod crdt;
mod error;
mod execution;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let sender = self.session.clone();
        let sender: UserMinimal = UserMinimal {
            id: sender.uid,
            name: sender.user.name,
            email: Some(sender.user.email),
            avatar_url: sender.user.avatar_url,
        };
        let room_id = self.room.clone();

        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<IncomingMessage>(&text) {
                Ok(IncomingMessage::Chat(chat_data)) => {
                    self.lobby.do_send(HandleChat {
                        chat_data,
                        sender,
                        room_id,
                    });
                }
                Ok(IncomingMessage::Crdt(crdt)) => {
                    self.lobby.do_send(HandleCrdtUpdate {
                        update: crdt.update,
                        sender,
                        room_id,
                    });
                }
                Ok(IncomingMessage::Ping) => {
                    let pong_msg = serde_json::json!({ "type": "pong" });
                    match serde_json::to_string(&pong_msg) {
                        Ok(text) => ctx.text(text),
                        Err(_) => ctx.text("error: failed to serialize pong"),
                    }
                }
                Err(_) => {
                    warn!("{:?} : {:?} : {:?}", text, sender.clone(), room_id);
                    ctx.text("error: invalid JSON");
                }
            },
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
