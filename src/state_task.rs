use crate::{
    config::Config,
    highlighter::Highlighter,
    non_empty_vec::NonEmptyVec,
    scraper::Item,
    state::{Job, Post, State},
};
use anyhow::{Context as _, Result, bail};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

pub(crate) struct StateTask;

impl StateTask {
    pub(crate) fn spawn(
        config: &Config,
    ) -> Result<(JoinHandle<()>, StateTaskCtl, Receiver<NonEmptyVec<Job>>)> {
        let (ctl_tx, ctl_rx) = tokio::sync::mpsc::channel(8);
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        let highlighter = Highlighter::new(config)?;
        let handle = tokio::spawn(async move { r#loop(tx, ctl_rx, highlighter).await });
        Ok((handle, StateTaskCtl(ctl_tx), rx))
    }
}

async fn r#loop(
    tx: Sender<NonEmptyVec<Job>>,
    mut rx: Receiver<StateTaskRequest>,
    highlighter: Highlighter,
) {
    let mut state = State::default();

    while let Some(req) = rx.recv().await {
        match req {
            StateTaskRequest::Set { root, items } => {
                if let Some(new_jobs) = state.set(root, items, &highlighter) {
                    if tx.send(new_jobs).await.is_err() {
                        log::error!("failed to send new jobs: channel is closed");
                        break;
                    }
                }
            }
            StateTaskRequest::Get { tx } => {
                let (post, jobs) = state.get();
                if tx.send((post, jobs)).is_err() {
                    log::error!("failed to reply to Get request: channel is closed");
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct StateTaskCtl(Sender<StateTaskRequest>);

impl StateTaskCtl {
    pub(crate) async fn set(&self, root: Item, items: Vec<Item>) {
        if self
            .0
            .send(StateTaskRequest::Set { root, items })
            .await
            .is_err()
        {
            log::error!("failed to send data to StateTask: channel is closed")
        }
    }
    pub(crate) async fn get(&self) -> Result<(Post, Vec<Job>)> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        if self.0.send(StateTaskRequest::Get { tx }).await.is_err() {
            bail!("failed to send a Get request to StateTask: channel is closed");
        }
        rx.await
            .context("failed to receive a Get response from StateTask: channel is closed")
    }
}

enum StateTaskRequest {
    Set {
        root: Item,
        items: Vec<Item>,
    },
    Get {
        tx: tokio::sync::oneshot::Sender<(Post, Vec<Job>)>,
    },
}
