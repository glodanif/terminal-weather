use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::printer::json_json::create_json_json_response;

pub struct JsonPrinter {}

impl JsonPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for JsonPrinter {
    fn print(&self, weather_data: WeatherData) {
        let json_json = create_json_json_response(weather_data);
        let result = serde_json::to_string_pretty(&json_json);
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

impl Default for JsonPrinter {
    fn default() -> Self {
        Self::new()
    }
}
