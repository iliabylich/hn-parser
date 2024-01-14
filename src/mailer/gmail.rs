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
    pub(crate) fn from_global_config() -> Self {
        let config = Config::global();
        let credentials = Credentials::from((&config.gmail_email, &config.gmail_password));

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(credentials)
            .build();
        Self { mailer }
    }

    async fn send_message(&self, message: Message) {
        match self.mailer.send(message).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn send_test_email(&self) {
        let message = Message::builder()
            .from("HN Jobs app <ibylich@gmail.com>".parse().unwrap())
            .to("Ilya Bylich <ibylich@gmail.com>".parse().unwrap())
            .subject("Test message from HN parser")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("HN parser is running."))
            .expect("Failed to build email message");

        self.send_message(message).await;
    }

    pub(crate) async fn send_html_email(&self, subject: &str, body: String) {
        let message = Message::builder()
            .from("HN Jobs app <ibylich@gmail.com>".parse().unwrap())
            .to("Ilya Bylich <ibylich@gmail.com>".parse().unwrap())
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
            .expect("Failed to build email message");

        self.send_message(message).await;
    }
}
