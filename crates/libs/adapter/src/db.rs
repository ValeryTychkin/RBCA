use std::time::Duration;

use crate::settings::SETTINGS;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

/// Static instance for managing a single database connection.
static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// Retrieves the static database connection, initializing it if necessary.
///
/// # Returns
/// A reference to the established database connection.
///
/// # Example
/// ```rust
/// use your_crate::db::get_connection;
///
/// #[tokio::main]
/// async fn main() {
///     let connection = get_connection().await;
///     // Use the connection here, e.g., perform queries
/// }
/// ```
pub async fn get_connection() -> &'static DatabaseConnection {
    DB.get_or_init(|| async { init().await }).await
}

async fn init() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(SETTINGS.database.url.to_owned());
    opt.max_connections(SETTINGS.database.max_connections.to_owned())
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(SETTINGS.database.debug.to_owned().into());

    Database::connect(opt).await.unwrap()
}
