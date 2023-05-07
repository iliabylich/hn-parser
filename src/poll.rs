use std::time::Duration;
use tokio::time::{interval, Interval};

use crate::config::Config;

pub(crate) struct Poll;

fn interval_from_config() -> Interval {
    let config = Config::global();
    interval(Duration::from_secs(config.poll_interval_in_seconds.into()))
}

impl Poll {
    pub(crate) async fn spawn() {
        let mut interval = interval_from_config();
        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick().await;
            }
        })
        .await
        .unwrap();
    }

    async fn tick() {
        println!("tick");
    }
}
