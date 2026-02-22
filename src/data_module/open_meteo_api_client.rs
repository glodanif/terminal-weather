use crate::data_module::data_module_error::DataModuleError;
use crate::data_module::location::Location;
use crate::data_module::weather_data::{CurrentWeather, WeatherData};
use serde::Deserialize;

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeatherResponse,
    hourly: HourlyWeatherResponse,
    daily: DailyWeatherResponse,
}

#[derive(Deserialize)]
struct CurrentWeatherResponse {
    temperature_2m: f32,
    wind_speed_10m: f32,
    apparent_temperature: f32,
    precipitation_probability: f32,
    cloud_cover: f32,
    weather_code: u8,
    is_day: u8,
    relative_humidity_2m: u8,
}

#[derive(Deserialize)]
pub struct HourlyWeatherResponse {
    pub time: Vec<String>,
    pub temperature_2m: Vec<Option<f32>>,
    pub wind_speed_10m: Vec<Option<f32>>,
    pub apparent_temperature: Vec<Option<f32>>,
    pub precipitation_probability: Vec<Option<f32>>,
    pub cloud_cover: Vec<Option<f32>>,
    pub weather_code: Vec<Option<u8>>,
    pub is_day: Vec<Option<u8>>,
    pub relative_humidity_2m: Vec<Option<u8>>,
}

#[derive(Deserialize)]
pub struct DailyWeatherResponse {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub weather_code: Vec<u8>,
}

const OPEN_METEO_API_URL: &str = "https://api.open-meteo.com/v1/forecast";
const OPEN_METEO_API_HOURLY_VALUES: &str = "temperature_2m,wind_speed_10m,apparent_temperature,weather_code,cloud_cover,precipitation_probability,is_day,relative_humidity_2m";
const OPEN_METEO_API_DAILY_VALUES: &str = "temperature_2m_max,temperature_2m_min,weather_code";

pub struct OpenMeteoApiClient {}

impl OpenMeteoApiClient {
    pub fn new() -> Self {
        OpenMeteoApiClient {}
    }

    pub fn get_weather_data(&self, location: Location) -> Result<WeatherData, DataModuleError> {
        let url = format!(
            "{}?latitude={}&longitude={}&current={}&daily={}&hourly={}&forecast_days=7&timezone=auto",
            OPEN_METEO_API_URL,
            location.latitude,
            location.longitude,
            OPEN_METEO_API_HOURLY_VALUES,
            OPEN_METEO_API_DAILY_VALUES,
            OPEN_METEO_API_HOURLY_VALUES
        );
        let response =
            reqwest::blocking::get(url).map_err(|_| DataModuleError::FailedToGetWeatherData)?;

        if response.status().is_success() {
            let response_json = response
                .json()
                .map_err(|_| DataModuleError::FailedToGetWeatherData)?;
            let open_meteo_data: OpenMeteoResponse = serde_json::from_value(response_json)
                .map_err(|_| DataModuleError::FailedToGetWeatherData)?;
            Ok(WeatherData {
                current_weather: CurrentWeather {
                    temperature: open_meteo_data.current.temperature_2m as f64,
                    wind_speed: open_meteo_data.current.wind_speed_10m,
                    apparent_temperature: open_meteo_data.current.apparent_temperature,
                    precipitation_probability: open_meteo_data.current.precipitation_probability,
                    cloud_cover: open_meteo_data.current.cloud_cover,
                    relative_humidity_2m: open_meteo_data.current.relative_humidity_2m,
                    weather_code: open_meteo_data.current.weather_code,
                    wmo_emoji: wmo_icon(
                        open_meteo_data.current.weather_code,
                        open_meteo_data.current.is_day == 1,
                    ),
                    is_day: open_meteo_data.current.is_day == 1,
                },
            })
        } else {
            Err(DataModuleError::FailedToGetWeatherData)
        }
    }
}

impl Default for OpenMeteoApiClient {
    fn default() -> Self {
        Self::new()
    }
}

fn wmo_icon(code: u8, is_day: bool) -> &'static str {
    match code {
        0 => {
            if is_day {
                "☀️"
            } else {
                "🌙"
            }
        }
        1 | 2 => "⛅",
        3 => "☁️",
        45 | 48 => "🌫️",
        51..=57 => "🌦️",
        61..=67 => "🌧️",
        71..=77 => "❄️",
        80..=82 => "🌧️",
        85 | 86 => "🌨️",
        95 => "⛈️",
        96 | 99 => "⛈️",
        _ => "",
    }
}
