use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::keyword::Keyword;

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
    pub(crate) keyword_regexes: Vec<Keyword>,

    // Gmail options
    pub(crate) gmail_email: String,
    pub(crate) gmail_password: String,

    // Mailer options
    pub(crate) send_email_once_every_seconds: u64,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

impl Config {
    fn read() -> String {
        let path = if cfg!(debug_assertions) {
            std::env::var("HNPARSER_CONFIG_PATH")
                .expect("No HNPARSER_CONFIG_PATH environment variable set")
        } else {
            String::from("/etc/hnparser.json")
        };

        std::fs::read_to_string(path).expect("failed to read config file")
    }

    pub(crate) fn load() {
        let mut config: Config =
            serde_json::from_str(&Config::read()).expect("Failed to parse JSON in the config file");
        config.build_keyword_regexes();
        CONFIG.set(config).expect("failed to set config");
    }

    fn build_keyword_regexes(&mut self) {
        for keyword in &self.keywords {
            self.keyword_regexes.push(Keyword::from(keyword))
        }
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().expect("config is not loaded")
    }
}
