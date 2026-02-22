use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::printer::json_printer::JsonPrinter;
use crate::presentation_module::printer::tui_printer::TuiPrinter;
use crate::presentation_module::printer::waybar_printer::WaybarPrinter;

mod printer;
mod wind_speed;
mod wmo;

pub enum PresentationMode {
    Tui,
    Json,
    Waybar,
}

pub struct PresentationModule {}

impl PresentationModule {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&self, weather_data: WeatherData, presentation_mode: PresentationMode) {
        match presentation_mode {
            PresentationMode::Waybar => WaybarPrinter::new().print(weather_data),
            PresentationMode::Json => JsonPrinter::new().print(weather_data),
            PresentationMode::Tui => TuiPrinter::new().print(weather_data),
        }
    }
}

impl Default for PresentationModule {
    fn default() -> Self {
        Self::new()
    }
}
