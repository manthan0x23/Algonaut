use common::id::ShortId;


pub struct Code {
    
}

fn generate_session_key_from_room_id(id: &str) -> ShortId {
    format!("algonaut:code:{}", id)
}
