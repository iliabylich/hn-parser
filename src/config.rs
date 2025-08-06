use anyhow::{Context as _, Result};
use serde::Deserialize;
use tokio::sync::OnceCell;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    // Server options
    pub(crate) port: u16,

    // HN options
    pub(crate) user_id: String,
    pub(crate) poll_interval_in_seconds: u64,

    // Parser options
    pub(crate) keywords: Vec<String>,

    // Gmail options
    pub(crate) gmail_email: String,
    pub(crate) gmail_password: String,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "config.toml";

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "/etc/hnparser.toml";

impl Config {
    pub(crate) async fn setup() -> Result<()> {
        let contents = tokio::fs::read_to_string(CONFIG_PATH)
            .await
            .context("failed to read config file")?;
        let config: Config = toml::from_str(&contents).context("failed to parse config file")?;
        CONFIG.set(config).context("failed to set global Config")?;
        Ok(())
    }

    pub(crate) fn global() -> &'static Self {
        CONFIG.get().expect("global Config is not set")
    }
}
