use futures::future::join_all;

use crate::{config::Config, job::Job, post::Post};

pub(crate) struct HnClient;

#[derive(serde::Deserialize, Debug)]
struct User {
    submitted: Vec<u64>,
}

fn user_url() -> String {
    let config = Config::global();
    format!(
        "https://hacker-news.firebaseio.com/v0/user/{}.json?print=pretty",
        config.user_id
    )
}

#[derive(serde::Deserialize, Debug)]
struct Item {
    id: u64,
    by: Option<String>,
    text: Option<String>,
    title: Option<String>,
    kids: Option<Vec<u64>>,
    time: i64,
}

fn item_url(hn_id: u64) -> String {
    format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
        hn_id
    )
}

impl HnClient {
    async fn get_user() -> User {
        reqwest::get(user_url())
            .await
            .unwrap()
            .json::<User>()
            .await
            .unwrap()
    }

    async fn get_item(hn_id: u64) -> Item {
        reqwest::get(item_url(hn_id))
            .await
            .unwrap()
            .json::<Item>()
            .await
            .unwrap()
    }

    async fn get_items(hn_ids: &[u64]) -> Vec<Item> {
        join_all(hn_ids.iter().map(|hn_id| Self::get_item(*hn_id))).await
    }

    async fn get_items_in_chunk_of(hn_ids: &[u64], chunk_size: usize) -> Vec<Item> {
        let mut items = Vec::new();
        for chunk in hn_ids.chunks(chunk_size) {
            items.extend(Self::get_items(chunk).await);
        }
        items
    }

    pub(crate) async fn get_latest_post() -> Post {
        let user = Self::get_user().await;
        for hn_id in &user.submitted {
            let post = Self::get_item(*hn_id).await;
            if let Some(title) = post.title {
                if title.contains("Ask HN: Who is hiring?") {
                    return Post {
                        hn_id: post.id as i64,
                        name: title,
                    };
                }
            }
        }
        panic!("Failed to get latest post")
    }

    pub(crate) async fn get_jobs_under(post: &Post, max_hn_id: u64) -> Vec<Job> {
        let post = Self::get_item(post.hn_id as u64).await;

        let mut comment_ids = post.kids.unwrap_or_default();
        comment_ids.sort();
        comment_ids.retain(|e| *e > max_hn_id);

        println!("Checking comments with ids {:?}", comment_ids);

        Self::get_items_in_chunk_of(&comment_ids, 20)
            .await
            .into_iter()
            .map(|comment| {
                let mut job = Job {
                    hn_id: comment.id as i64,
                    text: comment.text.unwrap_or_default(),
                    by: comment.by.unwrap_or_default(),
                    post_hn_id: post.id as i64,
                    time: comment.time,
                    interesting: false,
                };
                job.interesting = job.has_keywords();
                job
            })
            .collect::<Vec<_>>()
    }
}
