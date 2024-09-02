use lettre::{
    message::MessageBuilder, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::config::Config;

pub struct PalformMailClient {
    smtp: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
}

impl PalformMailClient {
    pub async fn new(config: Config) -> Self {
        let transport = if config.smtp_starttls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
        }
        .expect("Initialise relay")
        .port(config.smtp_port)
        .credentials(Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        ))
        .build();

        if !config.smtp_skip_startup_check {
            transport
                .test_connection()
                .await
                .expect("Failed to connect to SMTP");
        }

        Self {
            smtp: transport,
            from_address: config.smtp_from_address,
        }
    }

    pub fn get_email_builder(&self) -> MessageBuilder {
        Message::builder().from(
            format!("Palform <{}>", self.from_address)
                .parse()
                .expect("Parse static palform sender email"),
        )
    }

    pub async fn send_email(&self, email: Message) -> Result<(), lettre::transport::smtp::Error> {
        self.smtp.send(email).await.map(|_| ())
    }
}
