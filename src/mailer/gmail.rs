use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, MultiPart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::config::Config;

#[derive(Clone)]
pub(crate) struct Gmail {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Gmail {
    pub(crate) fn from_global_config() -> Result<Self> {
        let config = Config::global()?;
        let credentials = Credentials::from((&config.gmail_email, &config.gmail_password));

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .context("failed to construct mailer")?
            .credentials(credentials)
            .build();
        Ok(Self { mailer })
    }

    async fn send_message(&self, message: Message) {
        match self.mailer.send(message).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn send_test_email(&self) -> Result<()> {
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

        self.send_message(message).await;
        Ok(())
    }

    pub(crate) async fn send_html_email(&self, subject: &str, body: String) -> Result<()> {
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
}
