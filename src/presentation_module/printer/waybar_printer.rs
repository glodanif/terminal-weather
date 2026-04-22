use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::wind_speed::WindSpeed;
use crate::presentation_module::wmo::Wmo;
use serde::Serialize;

#[derive(Serialize)]
struct WaybarJson {
    text: String,
    tooltip: String,
}

pub struct WaybarPrinter {}

impl WaybarPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for WaybarPrinter {
    fn print(&self, weather_data: WeatherData) {
        let current_weather = weather_data.current;
        let current_weather_units = weather_data.current_units;
        let wmo = Wmo::new(current_weather.weather_code, current_weather.is_day == 1.0);
        let wind_speed = WindSpeed::new(current_weather.wind_speed_10m);
        let weather_text = format!(
            "{} {:.1}{}",
            wmo.nerd_character, current_weather.temperature_2m, current_weather_units.temperature_2m
        );
        let tooltip_text = format!(
            "{}\n{}\nTemperature: {:.1}{}\nFeels like: {:.1}{}\nPrecipitation: {}{}\nCloud cover: {}{}",
            wmo.description,
            wind_speed.description,
            current_weather.temperature_2m,
            current_weather_units.temperature_2m,
            current_weather.apparent_temperature,
            current_weather_units.apparent_temperature,
            current_weather.precipitation_probability,
            current_weather_units.precipitation_probability,
            current_weather.cloud_cover,
            current_weather_units.cloud_cover,
        );
        let waybar_json = WaybarJson {
            text: weather_text,
            tooltip: tooltip_text,
        };
        let result = serde_json::to_string(&waybar_json);
        match result {
            Ok(json_string) => println!("{}", json_string),
            Err(e) => {
                eprintln!("Failed to serialize weather data: {}", e);
                std::process::exit(1);
            }
        }
    }
}

impl Default for WaybarPrinter {
    fn default() -> Self {
        Self::new()
    }
}
