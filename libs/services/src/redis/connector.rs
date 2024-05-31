use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use general::get_redis_uri;

pub async fn connect() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(get_redis_uri()).unwrap();
    Pool::builder().build(manager).await.unwrap()
}
