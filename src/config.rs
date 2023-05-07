use serde::Deserialize;
use serde_json;
use tokio::sync::OnceCell;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    // Database options
    pub(crate) database_path: String,

    // HN options
    pub(crate) user_id: String,
    pub(crate) poll_interval_in_seconds: u8,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

impl Config {
    pub(crate) fn load() {
        let path = std::env::var("HNPARSER_CONFIG_PATH")
            .expect("No HNPARSER_CONFIG_PATH environment variable set");
        let config = std::fs::read_to_string(&path).expect("failed to read config file");
        let config: Config = serde_json::from_str(&config).unwrap();
        CONFIG.set(config).unwrap();
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().unwrap()
    }
}
