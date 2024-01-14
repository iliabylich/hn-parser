use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::highlighter::Highlighter;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    // Server options
    pub(crate) listen_on: u16,

    // Database options
    pub(crate) database_path: String,

    // HN options
    pub(crate) user_id: String,
    pub(crate) poll_interval_in_seconds: u64,

    // Parser options
    pub(crate) keywords: Vec<String>,
    #[serde(skip_deserializing)]
    pub(crate) highlighter: Highlighter,

    // Gmail options
    pub(crate) gmail_email: String,
    pub(crate) gmail_password: String,

    // Mailer options
    pub(crate) send_email_once_every_seconds: u64,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "config.json";

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "/etc/hnparser.json";

impl Config {
    pub(crate) fn load() {
        let file = std::fs::File::open(CONFIG_PATH).expect("Failed to open config file");
        let mut config = serde_json::from_reader::<_, Config>(file).unwrap();
        config.highlighter = Highlighter::new(&config.keywords);
        CONFIG.set(config).expect("failed to set config");
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().expect("config is not loaded")
    }
}
