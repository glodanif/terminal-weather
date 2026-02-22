mod cli;
mod data_module;
mod presentation_module;

use crate::cli::{Cli, Command};
use crate::data_module::Location;
use crate::presentation_module::{PresentationMode, PresentationModule};
use clap::Parser;
use data_module::DataModule;

fn main() {
    let data_module = DataModule::default();
    let presentation_module = PresentationModule::default();
    let cli = Cli::parse();
    match cli.command {
        None => {
            let presentation_mode = if cli.json {
                PresentationMode::Json
            } else if cli.waybar {
                PresentationMode::Waybar
            } else {
                PresentationMode::Tui
            };

            print_weather_data(&data_module, &presentation_module, presentation_mode);
        }
        Some(Command::SetLocation {
            latitude,
            longitude,
        }) => set_location(&data_module, latitude, longitude),
    }
}

fn print_weather_data(
    data_module: &DataModule,
    presentation_module: &PresentationModule,
    presentation_mode: PresentationMode,
) {
    let result = data_module.get_weather_data();
    match result {
        Ok(weather_data) => {
            presentation_module.print(weather_data, presentation_mode);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn set_location(data_module: &DataModule, latitude: f64, longitude: f64) {
    let result = data_module.set_location(Location {
        latitude,
        longitude,
    });
    match result {
        Ok(_) => println!("Location set"),
        Err(e) => {
            eprintln!("Failed to set location: {}", e);
            std::process::exit(1);
        }
    }
}
