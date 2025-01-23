use env_settings_derive::EnvSettings;
use once_cell::sync::Lazy;

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings {
    jwt: JWT::from_env().unwrap(),
});

pub struct Settings {
    pub jwt: JWT,
}

#[derive(EnvSettings)]
#[env_settings(case_insensitive, delay, prefix = "JWT_")]
pub struct JWT {
    #[env_settings(default = "secret")]
    pub secret: String,
}
