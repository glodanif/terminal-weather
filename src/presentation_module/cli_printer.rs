use crate::data_module::WeatherData;

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
        println!("{}", weather_data.current_weather.wmo_emoji);
        println!(
            "Temperature: {:.1}°C",
            weather_data.current_weather.temperature
        );
        println!(
            "Feels like: {:.1}°C",
            weather_data.current_weather.apparent_temperature
        );
        println!(
            "Wind speed: {:.1}km/h",
            weather_data.current_weather.wind_speed
        );
        println!(
            "Precipitation: {}%",
            weather_data.current_weather.precipitation_probability
        );
        println!("Cloud cover: {}%", weather_data.current_weather.cloud_cover);
        println!(
            "Humidity: {}%",
            weather_data.current_weather.relative_humidity_2m
        );
    }
}

impl Default for CliPrinter {
    fn default() -> Self {
        Self::new()
    }
}
