use env_settings_derive::EnvSettings;
use once_cell::sync::Lazy;
use util_lib::bool::env::Bool;

/// Static instance of `Settings` initialized lazily at runtime.
pub(crate) static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings {
    database: Database::from_env().unwrap(),
    redis: Redis::from_env().unwrap(),
    amqp: Amqp::from_env().unwrap(),
});

pub(crate) struct Settings {
    pub database: Database,
    pub redis: Redis,
    pub amqp: Amqp,
}

#[derive(EnvSettings)]
#[env_settings(case_insensitive, delay, prefix = "DATABASE_")]
pub(crate) struct Database {
    #[env_settings(default = "postgresql://root:root@localhost:5433/api")]
    pub url: String,
    #[env_settings(default = 1)]
    pub max_connections: u32,
    #[env_settings(default = 0)]
    pub debug: Bool,
}

#[derive(EnvSettings)]
#[env_settings(case_insensitive, delay, prefix = "REDIS_")]
pub(crate) struct Redis {
    #[env_settings(default = "redis://redis@localhost:6379/0")]
    pub url: String,
}

#[derive(EnvSettings)]
#[env_settings(case_insensitive, delay, prefix = "AMQP_")]
pub(crate) struct Amqp {
    #[env_settings(default = "amqp://root:root@localhost:5672/api")]
    pub url: String,
}
