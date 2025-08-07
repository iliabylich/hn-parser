use anyhow::{Context as _, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    // Server options
    pub(crate) port: u16,

    // HN options
    pub(crate) poll_interval_in_seconds: u64,

    // Parser options
    pub(crate) keywords: Vec<String>,

    // Gmail options
    pub(crate) gmail_email: String,
    pub(crate) gmail_password: String,
}

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "config.toml";

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "/etc/hnparser.toml";

impl Config {
    pub(crate) async fn read() -> Result<Self> {
        let contents = tokio::fs::read_to_string(CONFIG_PATH)
            .await
            .context("failed to read config file")?;
        toml::from_str(&contents).context("failed to parse config file")
    }

    pub(crate) fn poll_interval(&self) -> u64 {
        let mut result = self.poll_interval_in_seconds;
        if result == 0 {
            log::warn!("Polling is disabled");
            result = u64::MAX;
        }
        result
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("port", &self.port)
            .field("poll_interval_in_seconds", &self.poll_interval_in_seconds)
            .field("keywords", &self.keywords)
            .field("gmail_email", &self.gmail_email)
            .field("gmail_password", &"******")
            .finish()
    }
}
