use crate::data_module::open_meteo_api_client::{DailyWeatherResponse, HourlyWeatherResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub current_units: CurrentUnits,
    pub hourly: HourlyWeatherResponse,
    pub daily: DailyWeatherResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
    pub apparent_temperature: f32,
    pub precipitation_probability: f32,
    pub cloud_cover: f32,
    pub relative_humidity_2m: u8,
    pub weather_code: u8,
    pub is_day: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub temperature_2m: String,
    pub wind_speed_10m: String,
    pub apparent_temperature: String,
    pub precipitation_probability: String,
    pub cloud_cover: String,
    pub relative_humidity_2m: String,
    pub weather_code: String,
}

#[derive(Debug)]
struct HourlyForecast {
    pub forecast_items: [HourlyForecastItem; 24],
}

#[derive(Debug)]
struct HourlyForecastItem {
    pub time: String,
    pub temperature: f64,
}

#[derive(Debug)]
struct DailyForecast {
    pub forecast_items: [DailyForecastItem; 7],
}

#[derive(Debug)]
struct DailyForecastItem {
    pub date: String,
    pub temperature: f64,
}
