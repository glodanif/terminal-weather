use crate::data_module::WeatherData;
use crate::presentation_module::wind_speed::WindSpeed;
use crate::presentation_module::wmo::Wmo;

pub struct CliPrinter {}

impl CliPrinter {
    pub fn new() -> Self {
        CliPrinter {}
    }

    pub fn print_layout(&self, weather_data: WeatherData) {
        println!("---------------------------------");
        self.print_current_weather_data(weather_data);
        println!("---------------------------------");
    }

    fn print_current_weather_data(&self, weather_data: WeatherData) {
        let wmo = Wmo::new(
            weather_data.current.weather_code,
            weather_data.current.is_day == 1,
        );
        let wind_speed = WindSpeed::new(weather_data.current.wind_speed_10m);
        println!("{} | {}", wmo.emoji, wmo.description);
        println!(
            "Temperature: {:.1}{}",
            weather_data.current.temperature_2m, weather_data.current_units.temperature_2m
        );
        println!(
            "Feels like: {:.1}{}",
            weather_data.current.apparent_temperature,
            weather_data.current_units.apparent_temperature
        );
        println!(
            "Wind speed: {:.1}{} | {}",
            weather_data.current.wind_speed_10m,
            weather_data.current_units.wind_speed_10m,
            wind_speed.description
        );
        println!(
            "Precipitation: {}{}",
            weather_data.current.precipitation_probability,
            weather_data.current_units.precipitation_probability
        );
        println!(
            "Cloud cover: {}{}",
            weather_data.current.cloud_cover, weather_data.current_units.cloud_cover
        );
        println!(
            "Humidity: {}{}",
            weather_data.current.relative_humidity_2m,
            weather_data.current_units.relative_humidity_2m
        );
    }

    pub fn print_waybar(&self, weather_data: WeatherData) {
        let wmo = Wmo::new(
            weather_data.current.weather_code,
            weather_data.current.is_day == 1,
        );
        println!(
            "{} {:.1}{}",
            wmo.emoji,
            weather_data.current.temperature_2m,
            weather_data.current_units.temperature_2m
        );
    }

    pub fn print_json(&self, weather_data: WeatherData) {
        let result = serde_json::to_string_pretty(&weather_data.current);
        match result {
            Ok(json) => {
                println!("{}", json);
            }
            Err(e) => {
                eprintln!("Failed to serialize weather data: {}", e);
                std::process::exit(1);
            }
        }
    }
}

impl Default for CliPrinter {
    fn default() -> Self {
        Self::new()
    }
}
