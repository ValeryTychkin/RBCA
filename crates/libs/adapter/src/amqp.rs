use crate::settings::SETTINGS;

use lapin::{Connection, ConnectionProperties};
use tokio::sync::OnceCell;

/// Static instance for managing a single AMQP connection.
static AMQP: OnceCell<Connection> = OnceCell::const_new();

/// Retrieves the static AMQP connection, initializing it if necessary.
///
/// # Returns
/// A reference to the established AMQP connection.
///
/// # Example
/// ```rust
/// use your_crate::amqp::get_connection;
///
/// #[tokio::main]
/// async fn main() {
///     let connection = get_connection().await;
///     // Use the connection here, e.g., create a channel
/// }
/// ```
pub async fn get_connection() -> &'static Connection {
    AMQP.get_or_init(|| async { init().await }).await
}

pub async fn init() -> Connection {
    Connection::connect(&SETTINGS.amqp.url, ConnectionProperties::default())
        .await
        .unwrap()
}
