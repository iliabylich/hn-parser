use crate::{non_empty_vec::NonEmptyVec, state::Job, views::Views};
use anyhow::{Context as _, Result};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{MultiPart, header::ContentType},
    transport::smtp::authentication::Credentials,
};

#[derive(Debug)]
pub(crate) struct Mailer(AsyncSmtpTransport<Tokio1Executor>);

impl Mailer {
    pub(crate) fn new(credentials: Credentials) -> Result<Self> {
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
            .context("failed to construct mailer")?
            .port(587)
            .credentials(credentials)
            .build();
        Ok(Self(mailer))
    }

    async fn send_message(&self, message: Message) {
        match self.0.send(message).await {
            Ok(_) => log::info!("Email has been successfully sent!"),
            Err(e) => log::error!("failed to send an email: {e:?}"),
        }
    }

    async fn send_html_email(&self, subject: &str, body: String) -> Result<()> {
        let message = Message::builder()
            .from(
                "HN Jobs app <ibylich@gmail.com>"
                    .parse()
                    .context("failed to parse from")?,
            )
            .to("Ilya Bylich <ibylich@gmail.com>"
                .parse()
                .context("failed to parse to")?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        lettre::message::SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(String::from(
                                "Fallback body, if you see it then something went wrong.",
                            )),
                    )
                    .singlepart(
                        lettre::message::SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(body),
                    ),
            )
            .context("Failed to build email message")?;

        self.send_message(message).await;
        Ok(())
    }

    pub(crate) async fn send(&self, jobs: NonEmptyVec<Job>) -> Result<()> {
        let jobs = jobs.into_vec();
        let body = Views::jobs_email(jobs)?;
        self.send_html_email("New jobs from HN", body).await?;
        Ok(())
    }
}
