use lambda_runtime::{tracing, Error};
use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::WeatherService;
use infrastructure::{DiscordNotifier, WeatherApiClient};

const DEFAULT_LOCATION: &str = "Cenon";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let _ = dotenvy::dotenv();

    let service = Arc::new(build_service()?);

    #[cfg(feature = "local")]
    {
        service.fetch_and_notify().await?;
        return Ok(());
    }

    #[cfg(not(feature = "local"))]
    {
        use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
        use lambda_runtime::{run, service_fn, LambdaEvent};

        run(service_fn(move |event: LambdaEvent<CloudWatchEvent>| {
            let service = Arc::clone(&service);
            async move {
                tracing::info!("Payload: {:?}", event.payload);
                service
                    .fetch_and_notify()
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }))
        .await
    }
}

fn build_service() -> Result<WeatherService, Error> {
    let location =
        std::env::var("WEATHER_LOCATION").unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
    let notifier = DiscordNotifier::from_env()?;
    let weather_client = WeatherApiClient::new();

    Ok(WeatherService::new(weather_client, notifier, location))
}
