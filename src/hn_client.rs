use anyhow::{Context, Result};

use crate::config::Config;

pub(crate) struct HnClient;

#[derive(serde::Deserialize, Debug)]
struct User {
    submitted: Vec<u32>,
}

fn user_url() -> Result<String> {
    let config = Config::global()?;
    Ok(format!(
        "https://hacker-news.firebaseio.com/v0/user/{}.json?print=pretty",
        config.user_id
    ))
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Item {
    pub(crate) id: u32,
    pub(crate) by: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) title: Option<String>,
    kids: Option<Vec<u32>>,
    pub(crate) time: i64,
}

fn item_url(hn_id: u32) -> String {
    format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
        hn_id
    )
}

impl HnClient {
    async fn get_user() -> Result<User> {
        reqwest::get(user_url()?)
            .await
            .context("failed to get user response")?
            .json::<User>()
            .await
            .context("failed to parse user response")
    }

    async fn get_item(hn_id: u32) -> Result<Item> {
        reqwest::get(item_url(hn_id))
            .await
            .context("failed to get item")?
            .json::<Item>()
            .await
            .context("failed to parse item response")
    }

    async fn get_items(hn_ids: &[u32]) -> Result<Vec<Item>> {
        hn_ids
            .iter()
            .map(|hn_id| Self::get_item(*hn_id))
            .collect::<futures::future::JoinAll<_>>()
            .await
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()
    }

    async fn get_items_in_chunk_of(hn_ids: &[u32], chunk_size: usize) -> Result<Vec<Item>> {
        let mut items = Vec::with_capacity(hn_ids.len());
        for chunk in hn_ids.chunks(chunk_size) {
            items.extend(Self::get_items(chunk).await?);
        }
        Ok(items)
    }

    pub(crate) async fn get_latest_post() -> Result<Item> {
        let user = Self::get_user().await?;
        for hn_id in user.submitted {
            let post = Self::get_item(hn_id).await?;
            if let Some(title) = post.title.as_ref() {
                if title.contains("Ask HN: Who is hiring?") {
                    return Ok(post);
                }
            }
        }
        panic!("Failed to get latest post")
    }

    pub(crate) async fn get_jobs_under(post_hn_id: u32, after_job_id: u32) -> Result<Vec<Item>> {
        let mut comment_ids = Self::get_item(post_hn_id).await?.kids.unwrap_or_default();
        comment_ids.sort();
        comment_ids.retain(|e| *e > after_job_id);

        println!("Loading comments with ids {:?}", comment_ids);

        Self::get_items_in_chunk_of(&comment_ids, 20).await
    }
}
