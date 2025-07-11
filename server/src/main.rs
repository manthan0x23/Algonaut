use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::{Logger, from_fn},
    web::{self, scope},
};
use common::storage::AwsS3;
use std::env;
use tracing::{error, info};
use tracing_subscriber;

mod health_check;
mod routes;
mod utils;
use utils::app_state::AppState;
mod middlewares;
mod websocket;

use crate::{
    middlewares::auth_middleware,
    utils::app_state::AppEnv,
    websocket::{models::lobby::Lobby, ws_handler},
};

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

    let lobby: Addr<Lobby> = Lobby::new().start();

    let storage = match AwsS3::new(
        app_env.aws_region.clone(),
        app_env.aws_access_key.clone(),
        app_env.aws_secret_key.clone(),
        app_env.aws_s3_bucket_name.clone(),
        app_env.aws_cloud_front_distribution_url.clone(),
    )
    .await
    {
        Ok(store) => store,
        Err(err) => {
            error!("{:?}", err);
            panic!("{:?}", err);
        }
    };

    let app_state = web::Data::new(AppState {
        database: db,
        redis_pool: redis_pool,
        env: app_env.clone(),
        lobby: lobby,
        storage: storage,
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
            .service(
                scope("ws")
                    .wrap(from_fn(auth_middleware))
                    .route("{room_id}", web::get().to(ws_handler)),
            )
    })
    .bind(bind_server)?
    .worker_max_blocking_threads(1)
    .run()
    .await
}
