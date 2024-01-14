use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub(crate) hn_id: u32,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) post_hn_id: u32,
    pub(crate) time: i64,
    pub(crate) interesting: bool,
    pub(crate) email_sent: bool,
}

impl Job {
    pub(crate) fn highlight_keywords(mut self, highlight_fn: impl Fn(&str) -> String) -> Self {
        self.text = Config::global()
            .highlighter
            .highlight(std::mem::take(&mut self.text), |capture| {
                highlight_fn(capture)
            });

        self
    }
}
