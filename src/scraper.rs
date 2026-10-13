use crate::http_client::Client;
use anyhow::{Context as _, Result, bail};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct User {
    pub(crate) submitted: Vec<u32>,
}
impl User {
    pub(crate) async fn get(client: &Client) -> Result<Self> {
        client
            .get_json("https://hacker-news.firebaseio.com/v0/user/whoishiring.json?print=pretty")
            .await
            .context("failed to get user")
    }

    pub(crate) async fn last_item(&self, client: &Client) -> Result<Item> {
        for id in self.submitted.iter() {
            let item = Item::get(client, *id).await?;
            if let Some(title) = item.title.as_ref() {
                if title.contains("Ask HN: Who is hiring?") {
                    return Ok(item);
                }
            }
        }
        bail!("failed to find post")
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Item {
    pub(crate) id: u32,
    pub(crate) by: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) kids: Option<Vec<u32>>,
    pub(crate) time: i64,
}

impl Item {
    pub(crate) async fn get(client: &Client, id: u32) -> Result<Self> {
        client
            .get_json(&format!(
                "https://hacker-news.firebaseio.com/v0/item/{id}.json?print=pretty"
            ))
            .await
            .with_context(|| format!("failed to get item {id}"))
    }

    pub(crate) async fn get_many(client: &Client, ids: &[u32]) -> Result<Vec<Self>> {
        async fn get_concurrently(client: &Client, ids: &[u32]) -> Result<Vec<Item>> {
            ids.iter()
                .map(|id| Item::get(client, *id))
                .collect::<futures_util::future::JoinAll<_>>()
                .await
                .into_iter()
                .collect::<Result<Vec<_>>>()
        }

        let mut items = Vec::with_capacity(ids.len());
        for chunk in ids.chunks(20) {
            items.extend(get_concurrently(client, chunk).await?);
        }
        Ok(items)
    }
}
