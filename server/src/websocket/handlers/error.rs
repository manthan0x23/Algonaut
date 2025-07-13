use actix::Addr;

use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    outgoing::{OutgoingError, OutgoingMessage},
};

pub async fn send_error(ws: &Addr<WsConnection>, msg: &str) {
    let error = OutgoingMessage::Error(OutgoingError::new(msg.to_string(), vec![msg.to_string()]));

    if let Ok(json) = serde_json::to_string(&error) {
        let _ = ws.send(WsText(json));
    }
}
