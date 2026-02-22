use crate::data_module::WeatherData;
use crate::presentation_module::cli_printer::CliPrinter;

mod cli_printer;
mod wind_speed;
mod wmo;

pub enum PresentationMode {
    Cli,
    Json,
    Waybar,
}

pub struct PresentationModule {
    cli_printer: CliPrinter,
}

impl PresentationModule {
    pub fn new() -> Self {
        PresentationModule {
            cli_printer: CliPrinter::default(),
        }
    }

    pub fn print(&self, weather_data: WeatherData, presentation_mode: PresentationMode) {
        match presentation_mode {
            PresentationMode::Cli => {
                self.cli_printer.print_layout(weather_data);
            }
            PresentationMode::Json => {
                self.cli_printer.print_json(weather_data);
            }
            PresentationMode::Waybar => {
                self.cli_printer.print_waybar(weather_data);
            }
        }
    }
}

impl Default for PresentationModule {
    fn default() -> Self {
        Self::new()
    }
}
