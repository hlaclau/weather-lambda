use crate::domain::{
    Astronomy, CurrentWeather, DailyForecast, HourlyForecast, Temperature, WeatherReport, Wind,
};
use serde::Deserialize;

const BASE_URL: &str = "https://wttr.in";

/// Client for fetching weather data from wttr.in API.
pub struct WeatherApiClient {
    client: reqwest::Client,
}

impl WeatherApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch(&self, location: &str) -> Result<WeatherReport, WeatherApiError> {
        let url = format!("{}/{}?format=j1", BASE_URL, location);

        let response: ApiResponse = self
            .client
            .get(&url)
            .header("User-Agent", "weather-lambda")
            .send()
            .await?
            .json()
            .await?;

        Ok(self.to_domain(location, response))
    }

    fn to_domain(&self, location: &str, response: ApiResponse) -> WeatherReport {
        let current = &response.current_condition[0];
        let today = &response.weather[0];
        let astro = &today.astronomy[0];

        WeatherReport {
            location: location.to_string(),
            date: today.date.clone(),
            current: CurrentWeather {
                description: current.weather_desc[0].value.trim().to_string(),
                temperature: Temperature {
                    celsius: current.temp_c.parse().unwrap_or(0),
                    feels_like: current.feels_like_c.parse().unwrap_or(0),
                },
                humidity: current.humidity.parse().unwrap_or(0),
                wind: Wind {
                    speed_kmh: current.windspeed_kmph.parse().unwrap_or(0),
                    direction: current.winddir_16point.clone(),
                },
                precipitation_mm: current.precip_mm.parse().unwrap_or(0.0),
                uv_index: current.uv_index.parse().unwrap_or(0),
                pressure_hpa: current.pressure.parse().unwrap_or(0),
                visibility_km: current.visibility.parse().unwrap_or(0),
            },
            daily: DailyForecast {
                temp_min: today.mintemp_c.parse().unwrap_or(0),
                temp_max: today.maxtemp_c.parse().unwrap_or(0),
                temp_avg: today.avg_temp_c.parse().unwrap_or(0),
                astronomy: Astronomy {
                    sunrise: astro.sunrise.trim().to_string(),
                    sunset: astro.sunset.trim().to_string(),
                    moon_phase: astro.moon_phase.trim().to_string(),
                },
            },
            hourly: today.hourly.iter().map(|h| self.parse_hourly(h)).collect(),
        }
    }

    fn parse_hourly(&self, h: &ApiHourly) -> HourlyForecast {
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

        HourlyForecast {
            hour: hour.to_string(),
            temperature: h.temp_c.parse().unwrap_or(0),
            chance_of_rain: h.chance_of_rain.parse().unwrap_or(0),
            description: h.weather_desc[0].value.trim().to_string(),
        }
    }
}

impl Default for WeatherApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum WeatherApiError {
    Request(reqwest::Error),
}

impl std::fmt::Display for WeatherApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(e) => write!(f, "Weather API request failed: {}", e),
        }
    }
}

impl std::error::Error for WeatherApiError {}

impl From<reqwest::Error> for WeatherApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

// --- API DTOs (private, only used for deserialization) ---

#[derive(Deserialize)]
struct ApiResponse {
    current_condition: Vec<ApiCurrentCondition>,
    weather: Vec<ApiWeather>,
}

#[derive(Deserialize)]
struct ApiCurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: String,
    humidity: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ApiValueWrapper>,
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
struct ApiWeather {
    date: String,
    #[serde(rename = "maxtempC")]
    maxtemp_c: String,
    #[serde(rename = "mintempC")]
    mintemp_c: String,
    #[serde(rename = "avgtempC")]
    avg_temp_c: String,
    astronomy: Vec<ApiAstronomy>,
    hourly: Vec<ApiHourly>,
}

#[derive(Deserialize)]
struct ApiAstronomy {
    sunrise: String,
    sunset: String,
    moon_phase: String,
}

#[derive(Deserialize)]
struct ApiHourly {
    time: String,
    #[serde(rename = "tempC")]
    temp_c: String,
    #[serde(rename = "chanceofrain")]
    chance_of_rain: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ApiValueWrapper>,
}

#[derive(Deserialize)]
struct ApiValueWrapper {
    value: String,
}
