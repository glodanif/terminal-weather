use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub current_units: CurrentUnits,
    pub hourly: HourlyForecast,
    pub daily:DailyForecast,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature_2m: f32,
    pub apparent_temperature: f32,
    pub wind_speed_10m: f32,
    pub precipitation_probability: u8,
    pub cloud_cover: u8,
    pub relative_humidity_2m: u8,
    pub weather_code: u8,
    pub is_day: f32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub time: Vec<String>,
    pub temperature_2m: Vec<Option<f32>>,
    pub wind_speed_10m: Vec<Option<f32>>,
    pub apparent_temperature: Vec<Option<f32>>,
    pub precipitation_probability: Vec<Option<f32>>,
    pub cloud_cover: Vec<Option<f32>>,
    pub relative_humidity_2m: Vec<Option<u8>>,
    pub weather_code: Vec<Option<u8>>,
    pub is_day: Vec<Option<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<Option<f32>>,
    pub temperature_2m_min: Vec<Option<f32>>,
    pub weather_code: Vec<Option<u8>>,
}
