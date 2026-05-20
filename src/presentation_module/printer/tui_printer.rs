use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::wind_speed::WindSpeed;
use crate::presentation_module::wmo::Wmo;
use chrono::{Local, Timelike};

const HI_ON: &str = "\x1b[1;33m";
const HI_OFF: &str = "\x1b[0m";

pub struct TuiPrinter {}

impl TuiPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for TuiPrinter {
    fn print(&self, weather_data: WeatherData) {
        let col_width = compute_col_width(&weather_data);
        let divider = "-".repeat((col_width + 1) * 12 + 1);
        println!("{}", divider);
        print_current_weather_data(&weather_data);
        println!("{}", divider);
        print_hourly_forecast_data(&weather_data, col_width);
        println!("{}", divider);
    }
}

fn compute_col_width(weather_data: &WeatherData) -> usize {
    (0..12)
        .map(|i| {
            let a = i * 2;
            let b = a + 1;
            let parts: Vec<_> = weather_data.hourly.time[a].split('T').collect();
            [
                parts[1].chars().count(),
                match avg_opt(
                    weather_data.hourly.temperature_2m[a],
                    weather_data.hourly.temperature_2m[b],
                ) {
                    Some(t) => format!("{:.1}°", t).chars().count(),
                    None => 2,
                },
                match avg_opt(
                    weather_data.hourly.apparent_temperature[a],
                    weather_data.hourly.apparent_temperature[b],
                ) {
                    Some(v) => format!("{:.1}°", v).chars().count(),
                    None => 2,
                },
                match avg_opt(
                    weather_data.hourly.precipitation_probability[a],
                    weather_data.hourly.precipitation_probability[b],
                ) {
                    Some(p) => format!("{:.0}%", p).chars().count(),
                    None => 2,
                },
            ]
            .into_iter()
            .max()
            .unwrap()
        })
        .max()
        .unwrap()
        .max(1)
}

fn find_current_col(weather_data: &WeatherData) -> Option<usize> {
    let now = Local::now();
    let current_date = now.format("%Y-%m-%d").to_string();
    let current_slot = now.hour() as usize / 2;
    (0..12).find(|&i| {
        let parts: Vec<_> = weather_data.hourly.time[i * 2].split('T').collect();
        parts[0] == current_date && i == current_slot
    })
}

fn print_current_weather_data(weather_data: &WeatherData) {
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

fn avg_opt(a: Option<f32>, b: Option<f32>) -> Option<f32> {
    match (a, b) {
        (Some(x), Some(y)) => Some((x + y) / 2.0),
        (Some(x), None) | (None, Some(x)) => Some(x),
        (None, None) => None,
    }
}

fn max_opt_code(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.max(y)),
        (Some(x), None) | (None, Some(x)) => Some(x),
        (None, None) => None,
    }
}

fn print_hourly_forecast_data(weather_data: &WeatherData, col_width: usize) {
    let mut hours: Vec<String> = Vec::new();
    let mut emojis: Vec<String> = Vec::new();
    let mut temps: Vec<String> = Vec::new();
    let mut apparent_temps: Vec<String> = Vec::new();
    let mut precip_probs: Vec<String> = Vec::new();

    for i in 0..12 {
        let a = i * 2;
        let b = a + 1;

        let parts: Vec<_> = weather_data.hourly.time[a].split('T').collect();
        hours.push(parts[1].to_string());

        let weather_code = max_opt_code(
            weather_data.hourly.weather_code[a],
            weather_data.hourly.weather_code[b],
        );
        let is_day = weather_data.hourly.is_day[a].unwrap_or(0.0);
        emojis.push(match weather_code {
            None => "?".to_string(),
            Some(code) => Wmo::new(code, is_day == 1.0).nerd_character.to_string(),
        });

        temps.push(match avg_opt(
            weather_data.hourly.temperature_2m[a],
            weather_data.hourly.temperature_2m[b],
        ) {
            Some(t) => format!("{:.1}°", t),
            None => "?°".to_string(),
        });

        apparent_temps.push(match avg_opt(
            weather_data.hourly.apparent_temperature[a],
            weather_data.hourly.apparent_temperature[b],
        ) {
            Some(v) => format!("{:.1}°", v),
            None => "?°".to_string(),
        });

        precip_probs.push(match avg_opt(
            weather_data.hourly.precipitation_probability[a],
            weather_data.hourly.precipitation_probability[b],
        ) {
            Some(p) => format!("{:.0}%", p),
            None => "?%".to_string(),
        });
    }

    let highlight_col = find_current_col(weather_data);
    let total_width = (col_width + 1) * 12 + 1;

    // All cells use .chars().count() as display width (e.g. ° is 2 bytes but 1 column).
    // Icon cells pass an explicit display_width since nerd chars (1 col) and emoji (2 cols)
    // both appear as a single codepoint to .chars().count().
    let print_row = |cells: &[String], display_width: usize| {
        let row: String = cells
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                let dw = if display_width > 0 { display_width } else { c.chars().count() };
                let padding = col_width.saturating_sub(dw);
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                if Some(idx) == highlight_col {
                    format!(
                        "|{}{}{}{}{}",
                        " ".repeat(left_pad),
                        HI_ON,
                        c,
                        HI_OFF,
                        " ".repeat(right_pad)
                    )
                } else {
                    format!("|{}{}{}", " ".repeat(left_pad), c, " ".repeat(right_pad))
                }
            })
            .collect();
        println!("{}|", row);
    };

    let print_inner_sep = || println!("{}", "-".repeat(total_width));

    print_row(&hours, 0);
    print_inner_sep();
    print_row(&emojis, 1); // nerd font chars are 1 terminal column wide
    print_row(&temps, 0);
    print_row(&apparent_temps, 0);
    print_row(&precip_probs, 0);
}

impl Default for TuiPrinter {
    fn default() -> Self {
        Self::new()
    }
}
