use crate::{
    get_channel, get_properties as get_base_properties, publish as base_pablish,
    settings::SETTINGS, ContentType,
};
use lapin::{
    options::QueueDeclareOptions,
    types::{AMQPValue, FieldTable, LongString, ShortString},
    BasicProperties, Channel,
};
use tokio::sync::OnceCell;

/// A static `OnceCell` for initializing and storing the user event channel.
static USER_CHANEL: OnceCell<Channel> = OnceCell::const_new();

/// Retrieves the user event channel, initializing it if necessary.
///
/// # Returns
/// A reference to the static `Channel`.
async fn get_chanel() -> &'static Channel {
    USER_CHANEL
        .get_or_init(|| async { channel_init().await })
        .await
}

/// Initializes the user event channel with durable queue options.
async fn channel_init() -> Channel {
    let mut options = QueueDeclareOptions::default();
    options.durable = true;
    get_channel(&SETTINGS.user_event.queue, options).await
}

/// Enum representing the types of events for user actions.
#[derive(Debug)]
pub enum EventType {
    Create,
    Update,
    Delete,
}

impl EventType {
    /// Returns the header key used for identifying the event type in AMQP headers.
    pub fn get_header_key() -> String {
        "x-event".to_string()
    }

    /// Returns the header value for the specific event type.
    pub fn get_header_value(&self) -> String {
        match self {
            Self::Create => "user-create".to_string(),
            Self::Update => "user-update".to_string(),
            Self::Delete => "user-delete".to_string(),
        }
    }
}

/// Generates AMQP properties with headers for the given event type.
///
/// # Arguments
/// * `event_type` - The type of user event.
///
/// # Returns
/// A `BasicProperties` instance with headers configured for the event.
fn get_properties(event_type: EventType) -> BasicProperties {
    let properties = get_base_properties(ContentType::Json);
    let mut headers = FieldTable::default();
    headers.insert(
        ShortString::from("x-services"),
        AMQPValue::LongString(LongString::from("*".as_bytes())),
    );

    headers.insert(
        ShortString::from(EventType::get_header_key()),
        AMQPValue::LongString(LongString::from(event_type.get_header_value().as_bytes())),
    );

    properties.with_headers(headers)
}

/// Publishes a user event with the specified content and event type.
///
/// # Arguments
/// * `user_content` - The content of the user event.
/// * `event_type` - The type of event to publish.
///
/// # Example
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let user_content = "{\"id\": \"1234\", \"name\": \"John\"}".to_string();
///     your_crate::publish(&user_content, your_crate::EventType::Create).await;
///     // Event published successfully
/// }
/// ```
pub async fn publish(user_content: &String, event_type: EventType) {
    let channel = get_chanel().await;
    base_pablish(
        user_content,
        channel,
        &"".to_string(),
        &SETTINGS.user_event.queue,
        get_properties(event_type),
    )
    .await;
}
