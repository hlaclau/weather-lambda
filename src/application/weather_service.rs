use crate::infrastructure::{DiscordNotifier, WeatherApiClient};
use crate::presentation::format_weather_report;
use lambda_runtime::tracing;

/// Application service that orchestrates fetching weather and sending notifications.
pub struct WeatherService {
    weather_client: WeatherApiClient,
    notifier: DiscordNotifier,
    location: String,
}

impl WeatherService {
    pub fn new(weather_client: WeatherApiClient, notifier: DiscordNotifier, location: String) -> Self {
        Self {
            weather_client,
            notifier,
            location,
        }
    }

    pub async fn fetch_and_notify(&self) -> Result<(), ServiceError> {
        tracing::info!("Fetching weather for {}", self.location);

        let report = self.weather_client.fetch(&self.location).await?;
        let message = format_weather_report(&report);

        tracing::debug!("Formatted message:\n{}", message);

        self.notifier.send(&message).await?;

        tracing::info!("Weather update sent to Discord");
        Ok(())
    }
}

#[derive(Debug)]
pub enum ServiceError {
    Weather(crate::infrastructure::weather_api::WeatherApiError),
    Discord(crate::infrastructure::discord::DiscordError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weather(e) => write!(f, "{}", e),
            Self::Discord(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<crate::infrastructure::weather_api::WeatherApiError> for ServiceError {
    fn from(err: crate::infrastructure::weather_api::WeatherApiError) -> Self {
        Self::Weather(err)
    }
}

impl From<crate::infrastructure::discord::DiscordError> for ServiceError {
    fn from(err: crate::infrastructure::discord::DiscordError) -> Self {
        Self::Discord(err)
    }
}
