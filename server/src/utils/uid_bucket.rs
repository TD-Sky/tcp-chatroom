use once_cell::sync::Lazy;
use snowflake::SnowflakeIdBucket;
use tokio::sync::Mutex;

static UID_BUCKET: Lazy<Mutex<SnowflakeIdBucket>> =
    Lazy::new(|| Mutex::new(SnowflakeIdBucket::new(19, 19)));

pub async fn pop() -> i64 {
    UID_BUCKET.lock().await.get_id()
}
