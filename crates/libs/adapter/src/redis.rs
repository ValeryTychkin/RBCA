use crate::settings::SETTINGS;
use redis::aio::MultiplexedConnection;

use tokio::sync::OnceCell;

/// Static instance for managing a single Redis connection.
static REDIS: OnceCell<MultiplexedConnection> = OnceCell::const_new();

/// Retrieves the static Redis connection, initializing it if necessary.
///
/// # Returns
/// A cloned instance of the established Redis connection.
///
/// # Example
/// ```rust
/// use your_crate::redis::get_connection;
///
/// #[tokio::main]
/// async fn main() {
///     let connection = get_connection().await;
///     // Use the connection here, e.g., send commands to Redis
/// }
/// ```
pub async fn get_connection() -> MultiplexedConnection {
    let con = REDIS.get_or_init(|| async { init().await }).await;
    con.clone()
}

pub async fn init() -> MultiplexedConnection {
    let client = redis::Client::open(SETTINGS.redis.url.to_owned()).unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}
