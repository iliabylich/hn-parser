use std::time::Duration;
use tokio::time::{interval, Interval};

use crate::{config::Config, hn_client::HnClient, state::AppState};

pub(crate) struct Poll;

fn interval_from_config() -> Option<Interval> {
    let config = Config::global();
    if config.poll_interval_in_seconds == 0 {
        println!("Polling is disabled");
        return None;
    }
    Some(interval(Duration::from_secs(
        config.poll_interval_in_seconds.into(),
    )))
}

impl Poll {
    pub(crate) async fn spawn(state: AppState) -> Option<()> {
        let mut interval = interval_from_config()?;

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick(state.clone()).await;
            }
        })
        .await
        .expect("Failed to spawn poll task");

        Some(())
    }

    async fn tick(state: AppState) -> Option<()> {
        let post = HnClient::get_latest_post().await?;
        state.database.create_post_if_missing(&post).await;
        println!("Latest post: {:?}", post);

        let max_job_id = state.database.max_job_id().await;
        println!("Max job id: {:?}", max_job_id);

        let jobs = HnClient::get_jobs_under(&post, max_job_id).await;
        let mut created = 0;
        for job in jobs {
            if state.database.create_job(&job).await {
                created += 1;
            }
        }
        println!("Sync completed, created {} jobs", created);

        Some(())
    }
}
