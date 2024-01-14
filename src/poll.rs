use std::time::Duration;
use tokio::time::{interval, Interval};

use crate::{config::Config, hn_client::HnClient, job::Job, post::Post, state::AppState};

pub(crate) struct Poll;

fn interval_from_config() -> Option<Interval> {
    let config = Config::global();
    if config.poll_interval_in_seconds == 0 {
        println!("Polling is disabled");
        return None;
    }
    Some(interval(Duration::from_secs(
        config.poll_interval_in_seconds,
    )))
}

impl Poll {
    pub(crate) async fn spawn(state: AppState) {
        let mut interval = if let Some(interval) = interval_from_config() {
            interval
        } else {
            return;
        };

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick(state.clone()).await;
            }
        })
        .await
        .expect("Failed to spawn poll task");
    }

    async fn tick(state: AppState) -> Option<()> {
        let post = HnClient::get_latest_post().await?;
        println!("Latest post: {} / {:?}", post.id, post.title);
        state
            .database
            .create_post_if_missing(Post {
                hn_id: post.id,
                name: post.title.unwrap_or_default(),
            })
            .await;

        let max_job_id = state.database.max_job_id().await;
        println!("Max job id: {:?}", max_job_id);

        let mut created_count = 0;
        for item in HnClient::get_jobs_under(post.id, max_job_id).await {
            let by = item.by.unwrap_or_default();
            let text = item.text.unwrap_or_default();
            let interesting = Config::global().highlighter.can_highlight(&text);
            let job = Job {
                hn_id: item.id,
                text,
                by,
                post_hn_id: post.id,
                time: item.time,
                email_sent: false,
                interesting,
            };
            if state.database.create_job(&job).await {
                created_count += 1;
            }
        }
        println!("Sync completed, created {} jobs", created_count);

        Some(())
    }
}
