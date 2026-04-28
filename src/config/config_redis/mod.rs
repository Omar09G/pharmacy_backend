use redis::AsyncCommands;
use std::sync::LazyLock;

use base64::Engine;
use log::info;

static REDIS_CLIENT: LazyLock<parking_lot::RwLock<Option<redis::Client>>> =
    LazyLock::new(|| parking_lot::RwLock::new(None));

pub async fn init_redis(url: &str) -> Result<(), String> {
    info!("Initializing Redis client at {}", url);
    match redis::Client::open(url) {
        Ok(client) => {
            *REDIS_CLIENT.write() = Some(client);
            Ok(())
        }
        Err(e) => Err(format!("failed to create redis client: {}", e)),
    }
}

fn client() -> Option<redis::Client> {
    REDIS_CLIENT.read().clone()
}

pub async fn get_connection() -> Result<redis::aio::Connection, String> {
    if let Some(c) = client() {
        c.get_async_connection()
            .await
            .map_err(|e| format!("redis connection error: {}", e))
    } else {
        Err("redis client not initialized".to_string())
    }
}

pub async fn set_kv(key: &str, value: &[u8], ttl_secs: usize) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(value);
    let _: () = redis::cmd("SET")
        .arg(key)
        .arg(b64)
        .arg("EX")
        .arg(ttl_secs)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("redis set failed: {}", e))?;
    Ok(())
}

pub async fn get_kv(key: &str) -> Result<Option<Vec<u8>>, String> {
    let mut conn = get_connection().await?;
    let res: Option<String> = conn
        .get(key)
        .await
        .map_err(|e| format!("redis get failed: {}", e))?;
    if let Some(s) = res {
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(|e| format!("base64 decode failed: {}", e))?;
        Ok(Some(decoded))
    } else {
        Ok(None)
    }
}

pub async fn set_json<T: serde::Serialize>(
    key: &str,
    value: &T,
    ttl_secs: usize,
) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| format!("serde json encode: {}", e))?;
    set_kv(key, &bytes, ttl_secs).await
}

pub async fn get_json<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>, String> {
    if let Some(bytes) = get_kv(key).await? {
        let v =
            serde_json::from_slice::<T>(&bytes).map_err(|e| format!("serde json decode: {}", e))?;
        Ok(Some(v))
    } else {
        Ok(None)
    }
}

pub async fn del_key(key: &str) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let _del_count: i32 = redis::cmd("DEL")
        .arg(key)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("redis del failed: {}", e))?;
    Ok(())
}

pub async fn del_pattern(pattern: &str) -> Result<(), String> {
    let mut conn = get_connection().await?;
    // KEYS is acceptable for admin scripts / small deployments; for large keyspaces use SCAN.
    let keys: Vec<String> = conn
        .keys(pattern)
        .await
        .map_err(|e| format!("redis keys failed: {}", e))?;
    if !keys.is_empty() {
        let _del_count: i32 = redis::cmd("DEL")
            .arg(keys)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis del failed: {}", e))?;
    }
    Ok(())
}

pub async fn set_raw(key: &str, value: &str, ttl_secs: Option<usize>) -> Result<(), String> {
    let mut conn = get_connection().await?;
    if let Some(ttl) = ttl_secs {
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis set raw failed: {}", e))?;
    } else {
        let _: () = conn
            .set(key, value)
            .await
            .map_err(|e| format!("redis set raw failed: {}", e))?;
    }
    Ok(())
}

pub async fn get_raw(key: &str) -> Result<Option<String>, String> {
    let mut conn = get_connection().await?;
    let res: Option<String> = conn
        .get(key)
        .await
        .map_err(|e| format!("redis get raw failed: {}", e))?;
    Ok(res)
}

pub async fn incr_by(key: &str, delta: i64) -> Result<i64, String> {
    let mut conn = get_connection().await?;
    let res: i64 = conn
        .incr(key, delta)
        .await
        .map_err(|e| format!("redis incr failed: {}", e))?;
    Ok(res)
}

pub async fn incr_by_float_str(key: &str, delta_str: &str) -> Result<f64, String> {
    let mut conn = get_connection().await?;
    let res: f64 = redis::cmd("INCRBYFLOAT")
        .arg(key)
        .arg(delta_str)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("redis incrbyfloat failed: {}", e))?;
    Ok(res)
}

pub async fn expire(key: &str, ttl_secs: usize) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let _: i32 = redis::cmd("EXPIRE")
        .arg(key)
        .arg(ttl_secs)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("redis expire failed: {}", e))?;
    Ok(())
}

pub async fn enqueue_json<T: serde::Serialize>(list_key: &str, value: &T) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let payload = serde_json::to_string(value).map_err(|e| format!("serde json encode: {}", e))?;
    let _: i64 = redis::cmd("RPUSH")
        .arg(list_key)
        .arg(payload)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("redis rpush failed: {}", e))?;
    Ok(())
}

pub async fn set_session<T: serde::Serialize>(
    session_id: &str,
    value: &T,
    ttl_secs: usize,
) -> Result<(), String> {
    let key = format!("session:{}", session_id);
    set_json(&key, value, ttl_secs).await
}

pub async fn get_session<T: serde::de::DeserializeOwned>(
    session_id: &str,
) -> Result<Option<T>, String> {
    let key = format!("session:{}", session_id);
    get_json(&key).await
}

pub async fn del_session(session_id: &str) -> Result<(), String> {
    let key = format!("session:{}", session_id);
    del_key(&key).await
}
