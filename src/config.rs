use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::highlighter::Highlighter;
use anyhow::{Context, Result};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    // Server options
    pub(crate) listen_on: u16,

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
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "config.json";

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "/etc/hnparser.json";

#[derive(Debug)]
struct ConfigIsNotSetError {}
impl std::fmt::Display for ConfigIsNotSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "global Config is not set")
    }
}
impl std::error::Error for ConfigIsNotSetError {}

impl Config {
    pub(crate) fn setup() -> Result<()> {
        let file = std::fs::File::open(CONFIG_PATH).context("failed to open config file")?;
        let mut config =
            serde_json::from_reader::<_, Config>(file).context("failed to parse config file")?;
        config.highlighter = Highlighter::new(&config.keywords)?;
        CONFIG.set(config).context("failed to set config")?;
        Ok(())
    }

    pub(crate) fn global() -> Result<&'static Config> {
        if let Some(config) = CONFIG.get() {
            Ok(config)
        } else {
            Err(ConfigIsNotSetError {}.into())
        }
    }
}
