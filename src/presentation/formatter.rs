use crate::domain::WeatherReport;

/// Formats a weather report into a Discord-friendly message.
pub fn format_weather_report(report: &WeatherReport) -> String {
    let mut msg = String::new();

    // Header
    msg.push_str(&format!(
        "# :sunny: Météo {} — {}\n\n",
        report.location, report.date
    ));

    // Current conditions
    let current = &report.current;
    msg.push_str(&format!(
        "## Actuellement\n\
         **{}** | **{}°C** (ressenti {}°C)\n\
         Humidité : {}% · Vent : {} km/h {}\n\
         Précipitations : {} mm · UV : {} · Pression : {} hPa · Visibilité : {} km\n\n",
        current.description,
        current.temperature.celsius,
        current.temperature.feels_like,
        current.humidity,
        current.wind.speed_kmh,
        current.wind.direction,
        current.precipitation_mm,
        current.uv_index,
        current.pressure_hpa,
        current.visibility_km,
    ));

    // Daily summary
    let daily = &report.daily;
    msg.push_str(&format!(
        "## Journée\n\
         :arrow_down: **{}°C** — :arrow_up: **{}°C** (moy. {}°C)\n\
         :sunrise: {} · :city_sunset: {} · :crescent_moon: {}\n\n",
        daily.temp_min,
        daily.temp_max,
        daily.temp_avg,
        daily.astronomy.sunrise,
        daily.astronomy.sunset,
        daily.astronomy.moon_phase,
    ));

    // Hourly forecast table
    msg.push_str("## Prévisions horaires\n```\n");
    msg.push_str(&format!(
        "{:<7} {:>5}  {:>6}  {}\n",
        "Heure", "Temp", "Pluie", "Conditions"
    ));
    msg.push_str(&format!("{}\n", "-".repeat(50)));

    for h in &report.hourly {
        msg.push_str(&format!(
            "{:<7} {:>4}°C  {:>4}%   {}\n",
            h.hour, h.temperature, h.chance_of_rain, h.description
        ));
    }

    msg.push_str("```");
    msg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    #[test]
    fn test_format_weather_report() {
        let report = WeatherReport {
            location: "Cenon".to_string(),
            date: "2026-01-26".to_string(),
            current: CurrentWeather {
                description: "Light drizzle".to_string(),
                temperature: Temperature {
                    celsius: 10,
                    feels_like: 7,
                },
                humidity: 87,
                wind: Wind {
                    speed_kmh: 20,
                    direction: "SSE".to_string(),
                },
                precipitation_mm: 0.0,
                uv_index: 0,
                pressure_hpa: 998,
                visibility_km: 10,
            },
            daily: DailyForecast {
                temp_min: 7,
                temp_max: 12,
                temp_avg: 9,
                astronomy: Astronomy {
                    sunrise: "08:28 AM".to_string(),
                    sunset: "06:02 PM".to_string(),
                    moon_phase: "First Quarter".to_string(),
                },
            },
            hourly: vec![
                HourlyForecast {
                    hour: "00:00".to_string(),
                    temperature: 7,
                    chance_of_rain: 100,
                    description: "Patchy rain nearby".to_string(),
                },
                HourlyForecast {
                    hour: "12:00".to_string(),
                    temperature: 10,
                    chance_of_rain: 77,
                    description: "Patchy rain nearby".to_string(),
                },
            ],
        };

        let result = format_weather_report(&report);

        assert!(result.contains("Cenon"));
        assert!(result.contains("10°C"));
        assert!(result.contains("Light drizzle"));
        assert!(result.contains("08:28 AM"));
    }
}
