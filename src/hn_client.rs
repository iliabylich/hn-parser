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
        comment_ids.truncate(5);

        println!("Checking comments with ids {:?}", comment_ids);

        let mut comments = Vec::new();
        for hn_id in comment_ids {
            let comment = Self::get_item(hn_id).await;
            comments.push(Job {
                hn_id: comment.id as i64,
                text: comment.text.unwrap_or_default(),
                by: comment.by.unwrap_or_default(),
                post_hn_id: post.id as i64,
            });
        }
        comments
    }
}
