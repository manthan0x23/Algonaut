use redis::connect::RedisConnectionPool;
use sea_orm::DatabaseConnection;

use crate::utils::env::*;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub redis_pool: RedisConnectionPool,
    pub env: AppEnv,
}

#[derive(Clone, Debug)]
pub struct AppEnv {
    pub database_url: String,
    pub redis_url: String,
    pub address: String,
    pub port: u16,
    pub client_url: String,
    pub cargo_env: String,
    pub rust_log: String,
    pub aws_access_key: String,
    pub aws_secret_key: String,
    pub aws_region: String,
    pub aws_cloud_front_distribution_url: String,
    pub aws_s3_bucket_name: String,
    pub google_auth_client_id: String,
    pub google_auth_client_secret: String,
}

impl AppEnv {
    pub fn from_lazy() -> Self {
        Self {
            database_url: DATABASE_URL.clone(),
            redis_url: REDIS_URL.clone(),
            address: ADDRESS.clone(),
            port: *PORT,
            client_url: CLIENT_URL.clone(),
            cargo_env: CARGO_ENV.clone(),
            rust_log: RUST_LOG.clone(),
            aws_access_key: AWS_ACCESS_KEY.clone(),
            aws_secret_key: AWS_SECRET_KEY.clone(),
            aws_region: AWS_REGION.clone(),
            aws_cloud_front_distribution_url: AWS_CLOUD_FRONT_DISTRIBUTION_URL.clone(),
            aws_s3_bucket_name: AWS_S3_BUCKET_NAME.clone(),
            google_auth_client_id: GOOGLE_AUTH_CLIENT_ID.clone(),
            google_auth_client_secret: GOOGLE_AUTH_CLIENT_SECRET.clone(),
        }
    }
}
