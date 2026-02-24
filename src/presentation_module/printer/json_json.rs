use crate::data_module::WeatherData;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonJson {
    current: CurrentWeatherJson,
    forecast: ForecastJson,
    units: UnitsJson,
}

#[derive(Serialize)]
struct UnitsJson {
    temperature: String,
    wind_speed: String,
}

#[derive(Serialize)]
struct CurrentWeatherJson {
    temperature: f32,
    apparent_temperature: f32,
    wind_speed: f32,
    precipitation_probability: u8,
    cloud_cover: u8,
    relative_humidity: u8,
    weather_code: u8,
    is_day: bool,
}

#[derive(Serialize)]
struct ForecastJson {
    daily: Vec<DailyForecastJson>,
}

#[derive(Serialize)]
struct DailyForecastJson {
    time: String,
    temperature_2m_max: Option<f32>,
    temperature_2m_min: Option<f32>,
    weather_code: Option<u8>,
    hourly: Vec<HourlyForecastJson>,
}

#[derive(Serialize)]
struct HourlyForecastJson {
    time: String,
    temperature: f32,
    apparent_temperature: f32,
    wind_speed: f32,
    precipitation_probability: u8,
    cloud_cover: u8,
    relative_humidity: u8,
    weather_code: u8,
    is_day: bool,
}

pub fn create_json_json_response(weather_data: WeatherData) -> JsonJson {
    let units = weather_data.current_units;
    let current_weather = weather_data.current;
    let current_weather_json = CurrentWeatherJson {
        temperature: current_weather.temperature_2m,
        apparent_temperature: current_weather.apparent_temperature,
        wind_speed: current_weather.wind_speed_10m,
        precipitation_probability: current_weather.precipitation_probability,
        cloud_cover: current_weather.cloud_cover,
        relative_humidity: current_weather.relative_humidity_2m,
        weather_code: current_weather.weather_code,
        is_day: current_weather.is_day == 1,
    };

    let daily = weather_data.daily;
    let hourly = weather_data.hourly;

    let daily_forecasts: Vec<DailyForecastJson> = (0..daily.time.len())
        .map(|i| {
            // Each day has 24 hourly entries
            let hour_start = i * 24;
            let hour_end = (hour_start + 24).min(hourly.time.len());

            let hourly_forecasts: Vec<HourlyForecastJson> = (hour_start..hour_end)
                .filter_map(|h| {
                    Some(HourlyForecastJson {
                        time: hourly.time[h].clone(),
                        temperature: hourly.temperature_2m[h]?,
                        apparent_temperature: hourly.apparent_temperature[h]?,
                        wind_speed: hourly.wind_speed_10m[h]?,
                        precipitation_probability: hourly.precipitation_probability[h]? as u8,
                        cloud_cover: hourly.cloud_cover[h]? as u8,
                        relative_humidity: hourly.relative_humidity_2m[h]?,
                        weather_code: hourly.weather_code[h]?,
                        is_day: hourly.is_day[h]? == 1,
                    })
                })
                .collect();

            DailyForecastJson {
                time: daily.time[i].clone(),
                temperature_2m_max: daily.temperature_2m_max[i],
                temperature_2m_min: daily.temperature_2m_min[i],
                weather_code: daily.weather_code[i],
                hourly: hourly_forecasts,
            }
        })
        .collect();

    let units_json = UnitsJson {
        temperature: units.temperature_2m,
        wind_speed: units.wind_speed_10m,
    };

    JsonJson {
        current: current_weather_json,
        forecast: ForecastJson {
            daily: daily_forecasts,
        },
        units: units_json,
    }
}
