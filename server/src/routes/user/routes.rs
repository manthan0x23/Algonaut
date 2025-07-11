use actix_web::{
    middleware::from_fn,
    web::{self, scope},
};

use crate::{middlewares::auth_middleware, routes::user::handlers::get_presigned_url};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/rooms")
            .wrap(from_fn(auth_middleware))
            .route("presign-url", web::get().to(get_presigned_url)),
    );
}
