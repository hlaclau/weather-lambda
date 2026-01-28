/// Core weather domain model representing a complete weather report.
pub struct WeatherReport {
    pub location: String,
    pub date: String,
    pub current: CurrentWeather,
    pub daily: DailyForecast,
    pub hourly: Vec<HourlyForecast>,
}

/// Current weather conditions.
pub struct CurrentWeather {
    pub description: String,
    pub temperature: Temperature,
    pub humidity: u8,
    pub wind: Wind,
    pub precipitation_mm: f32,
    pub uv_index: u8,
    pub pressure_hpa: u16,
    pub visibility_km: u8,
}

/// Temperature with actual and feels-like values.
pub struct Temperature {
    pub celsius: i8,
    pub feels_like: i8,
}

/// Wind information.
pub struct Wind {
    pub speed_kmh: u16,
    pub direction: String,
}

/// Daily forecast summary.
pub struct DailyForecast {
    pub temp_min: i8,
    pub temp_max: i8,
    pub temp_avg: i8,
    pub astronomy: Astronomy,
}

/// Sunrise, sunset and moon phase.
pub struct Astronomy {
    pub sunrise: String,
    pub sunset: String,
    pub moon_phase: String,
}

/// Hourly forecast entry.
pub struct HourlyForecast {
    pub hour: String,
    pub temperature: i8,
    pub chance_of_rain: u8,
    pub description: String,
}
