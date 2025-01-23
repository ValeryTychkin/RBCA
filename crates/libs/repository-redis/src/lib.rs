use adapter_lib::redis::get_connection;
use redis::{AsyncCommands, AsyncIter, FromRedisValue, JsonAsyncCommands, ToRedisArgs};
use serde::Serialize;

/// Retrieves the value of a key from Redis.
///
/// This function attempts to get the value stored at the given `key` in Redis and
/// deserializes it into the provided type `V`. If no value is found, `None` is returned.
///
/// # Example
/// ```rust
/// let result: Option<String> = get("my_key".to_string()).await;
/// ```
pub async fn get<V: FromRedisValue>(key: String) -> Option<V> {
    let mut con = get_connection().await;
    con.get(key).await.unwrap()
}

/// Sets a value for a given key in Redis, with an optional expiration time.
///
/// This function sets the value of the given `key` to `value`, with an optional expiration time
/// specified in seconds (`ex_sec`). If no expiration is specified, the key will persist indefinitely.
///
/// # Example
/// ```rust
/// set("my_key".to_string(), "some_value", Some(3600)).await;
/// ```
pub async fn set<'a, V: ToRedisArgs + Send + Sync + 'a>(
    key: String,
    value: V,
    ex_sec: Option<u64>,
) {
    let mut con = get_connection().await;
    match ex_sec {
        Some(v) => {
            let _: () = con.set_ex(key, value, v).await.unwrap();
        }
        None => {
            let _: () = con.set(key, value).await.unwrap();
        }
    }
}

/// Checks if the specified key exists in Redis.
///
/// This function returns `true` if the key exists, or `false` if it does not.
///
/// # Example
/// ```rust
/// let exists = exist("my_key".to_string()).await;
/// ```
pub async fn exist(key: String) -> bool {
    let mut con = get_connection().await;
    con.exists(key).await.unwrap()
}

/// Deletes the specified key from Redis.
///
/// This function removes the key from Redis if it exists.
///
/// # Example
/// ```rust
/// del("my_key".to_string()).await;
/// ```
pub async fn del(key: String) {
    let mut con = get_connection().await;
    let _: () = con.del(key).await.unwrap();
}

/// Retrieves all keys in Redis that match the given pattern.
///
/// This function scans Redis for keys matching the provided pattern and returns a list of matching keys.
///
/// # Example
/// ```rust
/// let keys = get_keys("prefix:*".to_string()).await;
/// ```
pub async fn get_keys(pattern: String) -> Vec<String> {
    let mut con = get_connection().await;
    let mut res: Vec<String> = Vec::new();
    let mut scan_res: AsyncIter<String> = con.scan_match(pattern).await.unwrap();
    while let Some(key) = scan_res.next_item().await {
        res.push(key);
    }
    res
}

/// Deletes all keys in Redis that match the given pattern.
///
/// This function scans for keys matching the provided pattern and deletes them.
///
/// # Example
/// ```rust
/// del_keys("prefix:*".to_string()).await;
/// ```
pub async fn del_keys(pattern: String) {
    let mut con = get_connection().await;
    let mut scan_res: AsyncIter<String> = con.scan_match(pattern).await.unwrap();
    while let Some(key) = scan_res.next_item().await {
        del(key).await;
    }
}

/// Appends an item to a JSON array at a specified path in a Redis key.
///
/// This function updates a JSON array stored at a key by appending a value to it, using the provided
/// path to locate the array in the nested structure.
///
/// # Example
/// ```rust
/// let path = vec!["some", "nested", "path"];
/// let value = "new_value";
/// arr_append("my_key".to_string(), &path, &value).await;
/// ```
pub async fn arr_append<V: Serialize + Send + Sync>(key: String, path: &Vec<String>, value: &V) {
    let mut con = get_connection().await;

    let mut redis_path: Vec<String> = vec!["$".to_string()];
    for path_key in path {
        redis_path.push(path_key.to_owned());
    }

    let _: () = con
        .json_arr_append(key, redis_path.join("."), value)
        .await
        .unwrap();
}
