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
    fn read() -> String {
        let path = std::env::var("HNPARSER_CONFIG_PATH")
            .expect("No HNPARSER_CONFIG_PATH environment variable set");
        std::fs::read_to_string(&path).expect("failed to read config file")
    }

    pub(crate) fn load() {
        let mut config: Config = serde_json::from_str(&Config::read()).unwrap();
        config.build_keyword_regexes();
        CONFIG.set(config).unwrap();
    }

    fn build_keyword_regexes(&mut self) {
        for keyword in &self.keywords {
            let regex = format!("\\b{}\\b", keyword);
            let regex = regex::RegexBuilder::new(&regex)
                .case_insensitive(true)
                .build()
                .unwrap();
            self.keyword_regexes.push(regex)
        }
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().unwrap()
    }
}
