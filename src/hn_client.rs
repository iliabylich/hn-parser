use crate::{config::Config, post::Post};

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
    by: String,
    text: String,
    title: Option<String>,
    kids: Vec<u64>,
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
}
