use deadpool_redis::{
    Config, Connection, CreatePoolError, Pool, PoolError, Runtime, redis::AsyncCommands,
};

pub fn create_redis_pool(redis_url: &str) -> Result<Pool, CreatePoolError> {
    println!("{}", &redis_url);
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1))
}

pub async fn connect_via_pool(pool: &Pool) -> Result<Connection, PoolError> {
    let conn = pool.get().await?;
    Ok(conn)
}

pub async fn ping_redis(pool: &Pool) -> Result<(), PoolError> {
    let mut conn = connect_via_pool(pool).await?;

    let _: String = conn.ping().await?;

    Ok(())
}
