use env_settings_derive::EnvSettings;
use once_cell::sync::Lazy;

pub(crate) static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings {
    user_event: UserEvent::from_env().unwrap(),
});

pub(crate) struct Settings {
    pub user_event: UserEvent,
}

#[derive(EnvSettings)]
#[env_settings(case_insensitive, delay, prefix = "AMQP_USER_EVENT_")]
pub(crate) struct UserEvent {
    #[env_settings(default = "userv_event_queue")]
    pub queue: String,
}
