use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub(crate) hn_id: i64,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) post_hn_id: i64,
    pub(crate) time: i64,
    pub(crate) interesting: bool,
    pub(crate) email_sent: bool,
}

impl Job {
    pub(crate) fn has_keywords(&self) -> bool {
        Config::global()
            .keyword_regexes
            .iter()
            .any(|keyword| keyword.is_match(&self.text))
    }

    pub(crate) fn highlight_keywords<F>(&mut self, f: F)
    where
        F: Fn(&str) -> String,
    {
        Config::global()
            .keyword_regexes
            .iter()
            .for_each(|keyword| self.text = keyword.replace_all(&self.text, |capture| f(capture)));
    }
}
