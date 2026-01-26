#[cfg(not(feature = "local"))]
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
#[cfg(not(feature = "local"))]
use lambda_runtime::{Error, LambdaEvent};
use lambda_runtime::tracing;
use serde::Deserialize;

const WEATHER_URL: &str = "https://wttr.in/Cenon?format=j1";

#[derive(Deserialize)]
struct WeatherResponse {
    current_condition: Vec<CurrentCondition>,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: String,
    humidity: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ValueWrapper>,
    #[serde(rename = "windspeedKmph")]
    windspeed_kmph: String,
    #[serde(rename = "winddir16Point")]
    winddir_16point: String,
    #[serde(rename = "precipMM")]
    precip_mm: String,
    #[serde(rename = "uvIndex")]
    uv_index: String,
    pressure: String,
    visibility: String,
}

#[derive(Deserialize)]
struct Weather {
    date: String,
    #[serde(rename = "maxtempC")]
    maxtemp_c: String,
    #[serde(rename = "mintempC")]
    mintemp_c: String,
    #[serde(rename = "avgtempC")]
    avg_temp_c: String,
    astronomy: Vec<Astronomy>,
    hourly: Vec<Hourly>,
}

#[derive(Deserialize)]
struct Astronomy {
    sunrise: String,
    sunset: String,
    moon_phase: String,
}

#[derive(Deserialize)]
struct Hourly {
    time: String,
    #[serde(rename = "tempC")]
    temp_c: String,
    #[serde(rename = "chanceofrain")]
    chance_of_rain: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct ValueWrapper {
    value: String,
}

#[cfg(not(feature = "local"))]
pub(crate) async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    tracing::info!("Payload: {:?}", event.payload);
    fetch_and_notify().await
}

pub(crate) async fn fetch_and_notify() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let webhook_url =
        std::env::var("DISCORD_WEBHOOK_URL").map_err(|_| "DISCORD_WEBHOOK_URL not set")?;

    let client = reqwest::Client::new();

    tracing::info!("Fetching weather from {}", WEATHER_URL);
    let weather: WeatherResponse = client
        .get(WEATHER_URL)
        .header("User-Agent", "weather-lambda")
        .send()
        .await?
        .json()
        .await?;

    let message = format_weather(&weather);
    println!("{}", message);

    let payload = serde_json::json!({ "content": message });
    let resp = client.post(&webhook_url).json(&payload).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Discord webhook failed: {status} - {body}").into());
    }

    tracing::info!("Weather update sent to Discord");
    Ok(())
}

fn format_weather(weather: &WeatherResponse) -> String {
    let current = &weather.current_condition[0];
    let today = &weather.weather[0];
    let astro = &today.astronomy[0];

    let mut msg = String::new();

    msg.push_str(&format!("# :sunny: Météo Cenon — {}\n\n", today.date));

    msg.push_str(&format!(
        "## Actuellement\n\
         **{}** | **{}°C** (ressenti {}°C)\n\
         Humidité : {}% · Vent : {} km/h {}\n\
         Précipitations : {} mm · UV : {} · Pression : {} hPa · Visibilité : {} km\n\n",
        current.weather_desc[0].value.trim(),
        current.temp_c,
        current.feels_like_c,
        current.humidity,
        current.windspeed_kmph,
        current.winddir_16point,
        current.precip_mm,
        current.uv_index,
        current.pressure,
        current.visibility,
    ));

    msg.push_str(&format!(
        "## Journée\n\
         :arrow_down: **{}°C** — :arrow_up: **{}°C** (moy. {}°C)\n\
         :sunrise: {} · :city_sunset: {} · :crescent_moon: {}\n\n",
        today.mintemp_c,
        today.maxtemp_c,
        today.avg_temp_c,
        astro.sunrise.trim(),
        astro.sunset.trim(),
        astro.moon_phase.trim(),
    ));

    msg.push_str("## Prévisions horaires\n```\n");
    msg.push_str(&format!(
        "{:<7} {:>5}  {:>6}  {}\n",
        "Heure", "Temp", "Pluie", "Conditions"
    ));
    msg.push_str(&format!("{}\n", "-".repeat(50)));

    for h in &today.hourly {
        let hour = match h.time.as_str() {
            "0" => "00:00",
            "300" => "03:00",
            "600" => "06:00",
            "900" => "09:00",
            "1200" => "12:00",
            "1500" => "15:00",
            "1800" => "18:00",
            "2100" => "21:00",
            other => other,
        };

        msg.push_str(&format!(
            "{:<7} {:>4}°C  {:>4}%   {}\n",
            hour,
            h.temp_c,
            h.chance_of_rain,
            h.weather_desc[0].value.trim()
        ));
    }

    msg.push_str("```");
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_weather() {
        let json = r#"{
            "current_condition": [{
                "temp_C": "10", "FeelsLikeC": "7", "humidity": "87",
                "weatherDesc": [{"value": "Light drizzle"}],
                "windspeedKmph": "20", "winddir16Point": "SSE",
                "precipMM": "0.0", "uvIndex": "0", "pressure": "998", "visibility": "10"
            }],
            "nearest_area": [],
            "request": [],
            "weather": [{
                "date": "2026-01-26", "maxtempC": "12", "mintempC": "7", "avgtempC": "9",
                "astronomy": [{"sunrise": "08:28 AM", "sunset": "06:02 PM", "moon_phase": "First Quarter"}],
                "hourly": [{
                    "time": "0", "tempC": "7", "chanceofrain": "100",
                    "weatherDesc": [{"value": "Patchy rain nearby"}]
                }, {
                    "time": "1200", "tempC": "10", "chanceofrain": "77",
                    "weatherDesc": [{"value": "Patchy rain nearby"}]
                }]
            }]
        }"#;

        let weather: WeatherResponse = serde_json::from_str(json).unwrap();
        let result = format_weather(&weather);
        assert!(result.contains("Cenon"));
        assert!(result.contains("10°C"));
        assert!(result.contains("Light drizzle"));
        assert!(result.contains("08:28 AM"));
    }
}
