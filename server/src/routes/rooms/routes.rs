use crate::{
    middlewares::auth_middleware,
    routes::rooms::handlers::{
        create_room, get_global_rooms, get_room_chats_for_user, get_rooms_for_user, join_room,
    },
};
use actix_web::{
    middleware::from_fn,
    web::{self, scope},
};

pub fn room_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/rooms")
            .wrap(from_fn(auth_middleware))
            .route("/create", web::post().to(create_room))
            .route("/join", web::patch().to(join_room))
            .route("/user", web::get().to(get_rooms_for_user))
            .route("/global", web::get().to(get_global_rooms))
            .route("/chats", web::get().to(get_room_chats_for_user)),
    );
}
