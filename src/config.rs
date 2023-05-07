use serde::Deserialize;
use serde_json;
use tokio::sync::OnceCell;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    // HN options
    pub(crate) userId: String,
    pub(crate) pollIntervalInSeconds: u8,
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
