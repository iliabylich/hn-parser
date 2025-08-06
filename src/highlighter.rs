use crate::config::Config;
use anyhow::{Context as _, Result};
use tokio::sync::OnceCell;

#[derive(Debug, Default)]
pub(crate) struct Highlighter {
    regexes: Vec<regex::Regex>,
}

static HIGHLIGHTER: OnceCell<Highlighter> = OnceCell::const_new();

impl Highlighter {
    pub(crate) fn setup() -> Result<()> {
        let regexes = Config::global()
            .keywords
            .iter()
            .map(|string| {
                let regex = format!("\\b{}\\b", string);
                regex::RegexBuilder::new(&regex)
                    .case_insensitive(true)
                    .build()
                    .context("invalid regex")
            })
            .collect::<Result<Vec<_>>>()?;
        HIGHLIGHTER
            .set(Self { regexes })
            .context("failed to set global Highlighter")?;
        Ok(())
    }

    pub(crate) fn global() -> &'static Self {
        HIGHLIGHTER.get().expect("global Highlighter is not set")
    }
}

impl Highlighter {
    pub(crate) fn can_highlight(&self, text: &str) -> bool {
        self.regexes.iter().any(|regex| regex.is_match(text))
    }

    pub(crate) fn highlight(&self, text: &mut String, pre: &str, post: &str) {
        for re in self.regexes.iter() {
            *text = re
                .replace_all(text, |captures: &regex::Captures<'_>| {
                    format!("{pre}{}{post}", &captures[0])
                })
                .to_string();
        }
    }
}
