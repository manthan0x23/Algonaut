use std::env;

use lazy_static::lazy_static;

fn must_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} is required in the environment", key))
}

fn must_var_default<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(default)
}

fn must_parse<T: std::str::FromStr>(key: &str) -> T {
    env::var(key)
        .unwrap_or_else(|_| panic!("{} is required in the environment", key))
        .parse::<T>()
        .unwrap_or_else(|_| panic!("{} could not be parsed", key))
}

lazy_static! {
    static ref _DOTENV: () = {
        dotenv::dotenv().ok();
    };
    pub static ref DATABASE_URL: String = {
        *_DOTENV;
        must_var("DATABASE_URL")
    };
    pub static ref REDIS_URL: String = {
        *_DOTENV;
        must_var("REDIS_URL")
    };
    pub static ref ADDRESS: String = {
        *_DOTENV;
        must_var_default("ADDRESS", "0.0.0.0".to_string())
    };
    pub static ref PORT: u16 = {
        *_DOTENV;
        must_var_default("PORT", 5080)
    };
    pub static ref CLIENT_URL: String = {
        *_DOTENV;
        must_var("CLIENT_URL")
    };
    pub static ref CARGO_ENV: String = {
        *_DOTENV;
        must_var_default("CARGO_ENV", "dev".to_string())
    };
    pub static ref RUST_LOG: String = {
        *_DOTENV;
        must_var_default("RUST_LOG", "server=info".to_string())
    };
    pub static ref AWS_ACCESS_KEY: String = {
        *_DOTENV;
        must_var("AWS_ACCESS_KEY")
    };
    pub static ref AWS_SECRET_KEY: String = {
        *_DOTENV;
        must_var("AWS_SECRET_KEY")
    };
    pub static ref AWS_REGION: String = {
        *_DOTENV;
        must_var("AWS_REGION")
    };
    pub static ref AWS_CLOUD_FRONT_DISTRIBUTION_URL: String = {
        *_DOTENV;
        must_var("AWS_CLOUD_FRONT_DISTRIBUTION_URL")
    };
    pub static ref AWS_S3_BUCKET_NAME: String = {
        *_DOTENV;
        must_var("AWS_S3_BUCKET_NAME")
    };
    pub static ref GOOGLE_AUTH_CLIENT_ID: String = {
        *_DOTENV;
        must_var("GOOGLE_AUTH_CLIENT_ID")
    };
    pub static ref GOOGLE_AUTH_CLIENT_SECRET: String = {
        *_DOTENV;
        must_var("GOOGLE_AUTH_CLIENT_SECRET")
    };
}
