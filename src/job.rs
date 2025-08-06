use crate::{highlighter::Highlighter, hn_client::Item};
use anyhow::{Context as _, Result};

#[derive(Debug, Clone)]
pub(crate) struct Job {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) time: i64,
}

impl Job {
    pub(crate) fn highlight_keywords(mut self, pre: &str, post: &str) -> Self {
        Highlighter::global().highlight(&mut self.text, pre, post);
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
