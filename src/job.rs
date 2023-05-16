use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub(crate) hn_id: i64,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) post_hn_id: i64,
    pub(crate) time: i64,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            hn_id: Default::default(),
            text: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing Rust elit, sed RUST do eiusmod tempor incididunt ut labore et rust dolore magna aliqua.").repeat(15),
            by: String::from("Username"),
            post_hn_id: 12345,
            time: 1298888434,
        }
    }
}

impl Job {
    pub(crate) fn has_keywords(&self) -> bool {
        Config::global()
            .keywords
            .iter()
            .any(|keyword| self.text.to_lowercase().contains(&keyword.to_lowercase()))
    }

    pub(crate) fn highlight_keywords<F>(&mut self, f: F)
    where
        F: Fn(&str) -> String,
    {
        Config::global().keyword_regexes.iter().for_each(|regex| {
            self.text = regex
                .replace_all(&self.text, |captures: &regex::Captures| f(&captures[0]))
                .to_string()
        });
    }
}
