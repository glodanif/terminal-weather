use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long, short)]
    pub json: bool,

    #[arg(long, short)]
    pub waybar: bool,
}

#[derive(Subcommand)]
pub enum Command {
    SetLocation { latitude: f64, longitude: f64 },
}
