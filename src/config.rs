use std::sync::OnceLock;
use serde::Deserialize;
use serde_yaml;

#[derive(Deserialize)]
pub struct Config {
    pub google_calendar_credentials: GoogleCalendarCredentials,
    pub bot_token: String,
    pub notifiers: Vec<Notifiers>
}

#[derive(Deserialize)]
pub struct GoogleCalendarCredentials {
    pub client_secret: String,
    pub client_id: String,
}

#[derive(Deserialize)]
pub enum Event {

}

#[derive(Deserialize)]
pub struct Reminder {

}

#[derive(Deserialize)]
pub enum Action {}

#[derive(Deserialize)]
pub struct Notifiers {
    pub on: Event,
    pub reminders: Vec<Reminder>,
    pub actions: Vec<Action>
}

pub fn global() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let yaml = std::fs::read_to_string("config.yml").expect("Could not read config.yml");
        serde_yaml::from_str::<Config>(&yaml).expect("Invalid config.yml")
    })
}
