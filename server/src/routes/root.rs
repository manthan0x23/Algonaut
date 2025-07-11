use crate::{
    middlewares::auth_middleware,
    routes::{authentication::auth::auth_routes, rooms::room_routes, user::user_routes, verify},
};
use actix_web::{
    middleware::from_fn,
    web::{self, scope},
};

pub fn app_root(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/auth").configure(auth_routes).service(
            web::resource("/verify")
                .wrap(from_fn(auth_middleware))
                .route(web::get().to(verify)),
        ),
    )
    .configure(room_routes)
    .configure(user_routes);
}
