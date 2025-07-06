use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::Logger,
    web::{self, scope},
};
use std::env;
use tracing::{error, info};
use tracing_subscriber;

mod health_check;

mod routes;
mod utils;
use utils::app_state::AppState;
mod middlewares;

use crate::utils::app_state::AppEnv;

fn configure_env() {
    dotenv::dotenv().ok();

    // Set default log level if not already set
    if env::var_os("RUST_LOG").is_none() {
        unsafe {
            env::set_var("RUST_LOG", "debug,actix_web=info");
        }
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configure_env();

    let app_env = AppEnv::from_lazy();

    let bind_server = (app_env.address.clone(), app_env.port.clone());

    let db = match database::connect::connect_and_migrate(&app_env.database_url.clone()).await {
        Ok(conn) => {
            info!("Connected to PostgreSQL");
            conn
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {}", e);
            panic!("Postgres connection failed");
        }
    };

    let redis_pool = match redis::connect::create_redis_pool(&app_env.redis_url.clone()) {
        Ok(pool) => {
            info!("Created Redis pool");
            pool
        }
        Err(e) => {
            error!("Failed to create Redis pool: {}", e);
            panic!("Redis pool creation failed");
        }
    };

    if let Err(e) = redis::connect::ping_redis(&redis_pool).await {
        error!("Redis PING failed: {}", e);
        panic!("Unable to connect to Redis server");
    } else {
        info!("Connected to Redis server");
    }

    let app_state = web::Data::new(AppState {
        database: db,
        redis_pool: redis_pool,
        env: app_env.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&app_env.client_url)
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(health_check::health_check)
            .service(scope("api").configure(routes::app_root))
    })
    .workers(2)
    .bind(bind_server)?
    .run()
    .await
}
