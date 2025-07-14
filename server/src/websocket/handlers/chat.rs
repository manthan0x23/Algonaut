use crate::websocket::{
    handlers::error::send_error,
    models::{
        WsText,
        lobby::{HandleChat, Lobby},
        outgoing::{ChatMessageType, ChatType, OutgoingChat},
    },
};
use actix::{AsyncContext, Handler};
use common::id::{ShortId, short_id};
use database::entity::chat::{ActiveModel as ChatModel, Model as ChatModelFull};
use sea_orm::{ActiveModelTrait, Set};
use tracing::debug;

impl Handler<HandleChat> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: HandleChat, ctx: &mut Self::Context) -> Self::Result {
        let db = self.database.clone();
        let store = self.storage.clone();
        let sender = msg.sender.clone();
        let uid = ShortId::from(sender.id.clone());
        let chat_data = msg.chat_data.clone();

        let room_id = msg.room_id.clone();
        let (room, _doc) = match self.rooms.get(&room_id) {
            Some(r) => r.clone(),
            None => return,
        };

        ctx.spawn(actix::fut::wrap_future::<_, Self>(async move {
            let (sender_ws, _) = match room.get(&uid) {
                Some(ws_ref) => ws_ref.clone(),
                None => return,
            };

            let (text_opt, file_opt) = match chat_data.message_type {
                ChatMessageType::Text => {
                    let message_text = match &chat_data.text {
                        Some(text) if !text.trim().is_empty() => text.clone(),
                        _ => {
                            send_error(&sender_ws, "Text cannot be empty").await;
                            return;
                        }
                    };
                    (Some(message_text), None)
                }
                _ => {
                    let file_key = match &chat_data.url {
                        Some(key) if !key.trim().is_empty() => key.clone(),
                        _ => {
                            send_error(&sender_ws, "Url key cannot be empty").await;
                            return;
                        }
                    };
                    let file_url = store.get_cdn_url(file_key);
                    (None, Some(file_url))
                }
            };

            let active_chat = ChatModel {
                id: Set(short_id(None)),
                r#type: Set(chat_data.message_type.to_string()),
                text: Set(text_opt.clone()),
                file: Set(file_opt.clone()),
                room_id: Set(room_id.clone()),
                user_id: Set(uid.clone()),
                created_at: Set(chat_data.timestamp.naive_utc()),
            };

            let inserted: ChatModelFull = match active_chat.insert(&db).await {
                Ok(model) => model,
                Err(e) => {
                    send_error(&sender_ws, &format!("Database error: {}", e)).await;
                    return;
                }
            };

            let outgoing = OutgoingChat::new(inserted, sender, ChatType::Message);

            let json = match serde_json::to_string(&outgoing) {
                Ok(data) => data,
                Err(e) => {
                    send_error(&sender_ws, &format!("Serialization error: {}", e)).await;
                    return;
                }
            };

            for entry in room.iter() {
                let (_peer_uid, (ws_conn, mm)) = entry;
                debug!("TO :: {:?}", mm);
                if ws_conn.send(WsText(json.clone())).await.is_err() {
                    continue;
                }
            }
        }));
    }
}
