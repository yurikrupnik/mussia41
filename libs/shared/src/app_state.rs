use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use mongodb::Database;
// use std::sync::Arc;
// use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    // pub pool: PgPool,
    pub db: Database,
    pub redis: Pool<RedisConnectionManager>,
}

impl AppState {
    pub fn new(db: Database, redis: Pool<RedisConnectionManager>) -> Self {
        Self { db, redis }
    }
}
