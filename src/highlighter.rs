use crate::config::Config;
use anyhow::{Context as _, Result};
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub(crate) struct Highlighter {
    regexes: Arc<Vec<regex::Regex>>,
}

impl Highlighter {
    pub(crate) fn new(config: &Config) -> Result<Self> {
        let regexes = config
            .keywords
            .iter()
            .map(|string| {
                let regex = format!("\\b{string}\\b");
                regex::RegexBuilder::new(&regex)
                    .case_insensitive(true)
                    .build()
                    .context("invalid regex")
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            regexes: Arc::new(regexes),
        })
    }

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
