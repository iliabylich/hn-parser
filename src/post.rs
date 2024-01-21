use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::hn_client::Item;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Post {
    pub(crate) id: u32,
    pub(crate) title: String,
}

impl TryFrom<Item> for Post {
    type Error = anyhow::Error;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let title = item.title.context("no title")?;

        Ok(Self { id: item.id, title })
    }
}
