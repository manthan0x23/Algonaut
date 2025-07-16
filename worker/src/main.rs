mod services;

use anyhow::Result;
use dotenvy::dotenv;
use redis::AsyncCommands;
use services::{ContainerService, Cube, DatabaseService, S3Service, SocketConnectionManager};
use std::env;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let docker_image = env::var("DOCKER_USER_CONTAINER_IMAGE")?;
    let docker_network = env::var("DOCKER_NETWORK")?;

    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_async_connection().await?;

    let container_service = ContainerService::new(
        SocketConnectionManager,
        S3Service,
        DatabaseService,
        docker_image,
        docker_network,
    );

    loop {
        if let Ok(Some(data)) = conn.lpop::<_, Option<String>>("execution").await {
            if let Ok(cube) = serde_json::from_str::<Cube>(&data) {
                if let Err(e) = container_service.create_container(&cube, 0).await {
                    eprintln!("Error spinning container: {:?}", e);
                }
            } else {
                eprintln!("Invalid cube payload: {}", data);
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}
