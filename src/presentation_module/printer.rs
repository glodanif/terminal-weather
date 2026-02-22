use crate::data_module::WeatherData;

pub mod tui_printer;
pub mod json_printer;
pub mod waybar_printer;
mod json_json;

pub trait Printer {
    fn print(&self, weather_data: WeatherData);
}
