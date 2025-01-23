pub mod event;
pub mod settings;

use adapter_lib::amqp::get_connection;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{FieldTable, ShortString},
    BasicProperties, Channel,
};
use uuid::Uuid;

/// Enum representing different content types for messages.
#[derive(Debug)]
pub enum ContentType {
    Text,
    Json,
}

impl ContentType {
    /// Converts the content type to a string representation.
    ///
    /// # Returns
    /// A string representing the content type.
    ///
    /// # Example
    /// ```rust
    /// use your_crate::ContentType;
    ///
    /// let content_type = ContentType::Json;
    /// assert_eq!(content_type.to_string(), "application/json".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Self::Text => "text/plain".to_string(),
            Self::Json => "application/json".to_string(),
        }
    }

    /// Converts the content type to a `ShortString` for use with AMQP priorities.
    ///
    /// # Returns
    /// A `ShortString` representing the content type.
    ///
    /// # Example
    /// ```rust
    /// use your_crate::ContentType;
    ///
    /// let content_type = ContentType::Text;
    /// let priority_content_type = content_type.to_priority_content_type();
    /// assert_eq!(priority_content_type.as_str(), "text/plain");
    /// ```
    pub fn to_priority_content_type(&self) -> ShortString {
        ShortString::from(self.to_string())
    }
}

/// Generates a unique message ID.
///
/// # Returns
/// A `ShortString` containing a UUID as the message ID.
///
/// # Example
/// ```rust
/// use your_crate::get_message_id;
///
/// let message_id = get_message_id();
/// assert_eq!(message_id.len(), 36); // UUID length
/// ```
pub(crate) fn get_message_id() -> ShortString {
    ShortString::from(Uuid::new_v4().to_string())
}

/// Creates basic properties for an AMQP message.
pub(crate) fn get_properties(content_type: ContentType) -> BasicProperties {
    let mut properties = BasicProperties::default();

    properties = properties.with_type(content_type.to_priority_content_type());
    properties = properties.with_message_id(get_message_id());

    properties
}

/// Opens a channel and declares a queue.
///
/// # Arguments
/// * `queue_name` - The name of the queue.
/// * `options` - Options for queue declaration.
///
/// # Returns
/// A `Channel` instance.
///
/// # Example
/// ```rust
/// use your_crate::get_channel;
/// use lapin::options::QueueDeclareOptions;
///
/// #[tokio::main]
/// async fn main() {
///     let channel = get_channel(&"my_queue".to_string(), QueueDeclareOptions::default()).await;
///     // Channel is ready to use
/// }
/// ```
pub(crate) async fn get_channel(queue_name: &String, options: QueueDeclareOptions) -> Channel {
    let connection = get_connection().await;
    let channel = connection.create_channel().await.unwrap();
    channel
        .queue_declare(queue_name, options, FieldTable::default())
        .await
        .unwrap();
    channel
}

/// Publishes a message to a specified exchange and queue.
///
/// # Arguments
/// * `content` - The content of the message.
/// * `channel` - The AMQP channel.
/// * `exchange_name` - The name of the exchange.
/// * `queue_name` - The name of the queue.
/// * `properties` - Message properties.
///
/// # Example
/// ```rust
/// use your_crate::{get_channel, publish, get_properties, ContentType};
/// use lapin::options::QueueDeclareOptions;
///
/// #[tokio::main]
/// async fn main() {
///     let queue_name = "my_queue".to_string();
///     let exchange_name = "".to_string(); // Default exchange
///     let channel = get_channel(&queue_name, QueueDeclareOptions::default()).await;
///     let properties = get_properties(ContentType::Text);
///     let message = "Hello, world!".to_string();
///
///     publish(&message, &channel, &exchange_name, &queue_name, properties).await;
///     // Message published successfully
/// }
/// ```
pub(crate) async fn publish(
    content: &String,
    channel: &Channel,
    exchange_name: &String,
    queue_name: &String,
    properties: BasicProperties,
) {
    channel
        .basic_publish(
            exchange_name,
            queue_name,
            BasicPublishOptions::default(),
            content.as_bytes(),
            properties,
        )
        .await
        .unwrap()
        .await
        .unwrap();
}
