use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, MultiPart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tokio::sync::OnceCell;

use crate::{config::Config, job::Job, views::Views};

#[derive(Clone, Debug)]
pub(crate) struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

static MAILER: OnceCell<Mailer> = OnceCell::const_new();

impl Mailer {
    pub(crate) fn setup() -> Result<()> {
        let config = Config::global()?;
        let credentials = Credentials::from((&config.gmail_email, &config.gmail_password));

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
            .context("failed to construct mailer")?
            .port(587)
            .credentials(credentials)
            .build();
        let mailer = Self { mailer };
        MAILER.set(mailer).context("failed to set mailer")?;
        Ok(())
    }

    fn global() -> Result<&'static Self> {
        MAILER.get().context("global mailer is not set")
    }

    async fn send_message(&self, message: Message) {
        match self.mailer.send(message).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn send_test_email() -> Result<()> {
        let message = Message::builder()
            .from(
                "HN Jobs app <ibylich@gmail.com>"
                    .parse()
                    .context("failed to parse from")?,
            )
            .to("Ilya Bylich <ibylich@gmail.com>"
                .parse()
                .context("failed to parse to")?)
            .subject("Test message from HN parser")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("HN parser is running."))
            .context("Failed to build email message")?;

        Self::global()?.send_message(message).await;
        Ok(())
    }

    async fn send_html_email(subject: &str, body: String) -> Result<()> {
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

        Self::global()?.send_message(message).await;
        Ok(())
    }

    pub(crate) async fn send_jobs_email(jobs: &[Job]) -> Result<()> {
        if jobs.is_empty() {
            return Ok(());
        }

        let body = Views::jobs_email(jobs)?;
        Self::send_html_email("New jobs from HN", body).await?;
        Ok(())
    }
}
