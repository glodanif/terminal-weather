use crate::data_module::WeatherData;
use crate::presentation_module::cli_printer::CliPrinter;

mod cli_printer;

pub struct PresentationModule {
    cli_printer: CliPrinter,
}

impl PresentationModule {
    pub fn new() -> Self {
        PresentationModule {
            cli_printer: CliPrinter::default(),
        }
    }

    pub fn print_layout(&self, weather_data: WeatherData) {
        self.cli_printer.print_layout(weather_data);
    }

    pub fn print_json(&self, weather_data: WeatherData) {
        let result = serde_json::to_string_pretty(&weather_data.current_weather);
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

impl Default for PresentationModule {
    fn default() -> Self {
        Self::new()
    }
}
