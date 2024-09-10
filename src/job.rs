use anyhow::{Context, Result};

use crate::{config::Config, hn_client::Item};

#[derive(Debug, Clone)]
pub(crate) struct Job {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) time: i64,
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

impl TryFrom<Item> for Job {
    type Error = anyhow::Error;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let text = item.text.context("no text")?;
        let by = item.by.unwrap_or_default();
        let time = item.time;

        Ok(Self {
            id: item.id,
            text,
            by,
            time,
        })
    }
}
