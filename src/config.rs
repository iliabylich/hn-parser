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

    // Parser options
    pub(crate) keywords: Vec<String>,
    #[serde(skip_deserializing)]
    pub(crate) keyword_regexes: Vec<regex::Regex>,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

impl Config {
    pub(crate) fn load() {
        let path = std::env::var("HNPARSER_CONFIG_PATH")
            .expect("No HNPARSER_CONFIG_PATH environment variable set");
        let config = std::fs::read_to_string(&path).expect("failed to read config file");
        let mut config: Config = serde_json::from_str(&config).unwrap();
        for keyword in &config.keywords {
            let regex = regex::RegexBuilder::new(keyword)
                .case_insensitive(true)
                .build()
                .unwrap();
            config.keyword_regexes.push(regex)
        }
        CONFIG.set(config).unwrap();
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().unwrap()
    }
}
