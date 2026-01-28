/// Client for sending notifications to Discord via webhook.
pub struct DiscordNotifier {
    client: reqwest::Client,
    webhook_url: String,
}

impl DiscordNotifier {
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            webhook_url,
        }
    }

    pub fn from_env() -> Result<Self, DiscordError> {
        let webhook_url =
            std::env::var("DISCORD_WEBHOOK_URL").map_err(|_| DiscordError::MissingWebhookUrl)?;
        Ok(Self::new(webhook_url))
    }

    pub async fn send(&self, message: &str) -> Result<(), DiscordError> {
        let payload = serde_json::json!({ "content": message });

        let response = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(DiscordError::WebhookFailed { status, body });
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum DiscordError {
    MissingWebhookUrl,
    Request(reqwest::Error),
    WebhookFailed {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingWebhookUrl => write!(f, "DISCORD_WEBHOOK_URL not set"),
            Self::Request(e) => write!(f, "Discord request failed: {}", e),
            Self::WebhookFailed { status, body } => {
                write!(f, "Discord webhook failed: {} - {}", status, body)
            }
        }
    }
}

impl std::error::Error for DiscordError {}

impl From<reqwest::Error> for DiscordError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}
