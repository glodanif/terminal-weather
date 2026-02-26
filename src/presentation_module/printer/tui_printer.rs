use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::wind_speed::WindSpeed;
use crate::presentation_module::wmo::Wmo;

pub struct TuiPrinter {}

impl TuiPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for TuiPrinter {
    fn print(&self, weather_data: WeatherData) {
        println!("---------------------------------");
        print_current_weather_data(weather_data);
        println!("---------------------------------");
    }
}

fn print_current_weather_data(weather_data: WeatherData) {
    let wmo = Wmo::new(
        weather_data.current.weather_code,
        weather_data.current.is_day == 1.0,
    );
    let wind_speed = WindSpeed::new(weather_data.current.wind_speed_10m);
    println!("{} | {}", wmo.emoji, wmo.description);
    println!(
        "Temperature: {:.1}{}",
        weather_data.current.temperature_2m, weather_data.current_units.temperature_2m
    );
    println!(
        "Feels like: {:.1}{}",
        weather_data.current.apparent_temperature, weather_data.current_units.apparent_temperature
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
        weather_data.current.relative_humidity_2m, weather_data.current_units.relative_humidity_2m
    );
}

impl Default for TuiPrinter {
    fn default() -> Self {
        Self::new()
    }
}
