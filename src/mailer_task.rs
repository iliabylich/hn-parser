use crate::{config::Config, mailer::Mailer, non_empty_vec::NonEmptyVec, state::Job};
use lettre::transport::smtp::authentication::Credentials;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

pub(crate) struct MailerTask;

impl MailerTask {
    pub(crate) fn spawn(config: &Config) -> (JoinHandle<()>, MailerTaskCtl) {
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        let credentials = Credentials::from((&config.gmail_email, &config.gmail_password));
        let handle = tokio::spawn(async move { r#loop(rx, credentials).await });
        (handle, MailerTaskCtl(tx))
    }
}

async fn r#loop(mut rx: Receiver<NonEmptyVec<Job>>, credentials: Credentials) {
    let mailer = match Mailer::new(credentials) {
        Ok(mailer) => mailer,
        Err(err) => {
            log::error!("failed to create mailer: {err:?}");
            return;
        }
    };

    while let Some(jobs) = rx.recv().await {
        if let Err(err) = mailer.send(jobs).await {
            log::error!("{err:?}");
        }
    }
}

pub(crate) struct MailerTaskCtl(Sender<NonEmptyVec<Job>>);

impl MailerTaskCtl {
    pub(crate) async fn send(&self, jobs: NonEmptyVec<Job>) {
        if self.0.send(jobs).await.is_err() {
            log::error!("failed to send jobs to Mailer task: channel is closed");
        }
    }
}
