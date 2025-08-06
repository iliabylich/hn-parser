use crate::{
    config::Config,
    scraper::{Item, User},
};
use anyhow::{Context as _, Result};
use reqwest::Client;
use std::time::Duration;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
    time::{Interval, interval},
};

pub(crate) struct ScraperTask;

impl ScraperTask {
    pub(crate) fn spawn(config: &Config) -> (JoinHandle<()>, Receiver<(Item, Vec<Item>)>) {
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        let delay = interval(Duration::from_secs(config.poll_interval()));
        let handle = tokio::spawn(async move { r#loop(tx, delay).await });
        (handle, rx)
    }
}

async fn r#loop(tx: Sender<(Item, Vec<Item>)>, mut delay: Interval) {
    let client = Client::new();

    loop {
        delay.tick().await;
        if let Err(err) = fetch(&client, &tx).await {
            log::error!("{err:?}");
        }
    }
}

async fn fetch(client: &Client, tx: &Sender<(Item, Vec<Item>)>) -> Result<()> {
    let user = User::get(client).await?;
    let root_item = user.last_item(client).await?;
    let kids = root_item.kids.clone().unwrap_or_default();
    let items = Item::get_many(client, &kids).await?;
    tx.send((root_item, items))
        .await
        .context("channel is closed")?;
    Ok(())
}
