use actix::{ActorContext, AsyncContext, StreamHandler, fut::wrap_future};
use actix_web_actors::ws;
use common::types::session::UserMinimal;
use std::time::Instant;

use crate::websocket::models::{
    connection::WsConnection, incomming::IncomingMessage, lobby::HandleChat,
};

mod chat;
mod connection;
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
        let lobby_addr = self.lobby.clone();
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
                    ctx.spawn(wrap_future(async move {
                        let _ = lobby_addr
                            .send(HandleChat {
                                chat_data,
                                sender,
                                room_id,
                            })
                            .await;
                    }));
                }
                Err(_) => ctx.text("error: invalid JSON"),
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
