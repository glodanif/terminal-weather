use serde::Serialize;

#[derive(Debug)]
pub struct WeatherData {
    pub current_weather: CurrentWeather,
    //pub today_forecast: HourlyForecast,
    //pub next_7_days_forecast: [DailyForecast; 7],
}

#[derive(Debug, Serialize)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub wind_speed: f32,
    pub apparent_temperature: f32,
    pub precipitation_probability: f32,
    pub cloud_cover: f32,
    pub relative_humidity_2m: u8,
    pub weather_code: u8,
    pub wmo_emoji: &'static str,
    pub is_day: bool,
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
