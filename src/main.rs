mod cli;
mod data_module;
mod presentation_module;

use crate::cli::{Cli, Command};
use crate::data_module::Location;
use crate::presentation_module::PresentationModule;
use clap::Parser;
use data_module::DataModule;

fn main() {
    let data_module = DataModule::default();
    let presentation_module = PresentationModule::default();
    let cli = Cli::parse();
    match cli.command {
        None => {
            print_weather_data(&data_module, &presentation_module, cli.json);
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
    as_json: bool,
) {
    let result = data_module.get_weather_data();
    match result {
        Ok(weather_data) => {
            if as_json {
                presentation_module.print_json(weather_data);
            } else {
                presentation_module.print_layout(weather_data);
            }
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
