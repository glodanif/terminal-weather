use crate::data_module::WeatherData;
use crate::presentation_module::printer::Printer;
use crate::presentation_module::wind_speed::WindSpeed;
use crate::presentation_module::wmo::Wmo;

pub struct TuiPrinter {}

impl TuiPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for TuiPrinter {
    fn print(&self, weather_data: WeatherData) {
        println!("---------------------------------");
        print_current_weather_data(&weather_data);
        println!("---------------------------------");
        print_hourly_forecast_data(&weather_data);
        println!("---------------------------------");
    }
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

fn print_hourly_forecast_data(weather_data: &WeatherData) {
    let mut hours: Vec<String> = Vec::new();
    let mut emojis: Vec<String> = Vec::new();
    let mut temps: Vec<String> = Vec::new();
    let mut apparent_temps: Vec<String> = Vec::new();
    let mut precip_probs: Vec<String> = Vec::new();

    for i in 0..24 {
        let parts: Vec<_> = weather_data.hourly.time[i].split('T').collect();
        hours.push(parts[1].to_string());

        let weather_code = weather_data.hourly.weather_code[i];
        let is_day = weather_data.hourly.is_day[i].unwrap_or(0.0);
        emojis.push(match weather_code {
            None => "?".to_string(),
            Some(code) => Wmo::new(code, is_day == 1.0).emoji.to_string(),
        });

        temps.push(match weather_data.hourly.temperature_2m[i] {
            Some(t) => format!("{:.1}°", t),
            None => "?°".to_string(),
        });

        apparent_temps.push(match weather_data.hourly.apparent_temperature[i] {
            Some(a) => format!("{:.1}°", a),
            None => "?°".to_string(),
        });

        precip_probs.push(match weather_data.hourly.precipitation_probability[i] {
            Some(p) => format!("{}%", p),
            None => "?%".to_string(),
        });
    }

    // All non-emoji rows are pure ASCII, so .len() == display width.
    // Emojis are all natively 2 terminal columns wide.
    let col_width = (0..24)
        .map(|i| {
            [&hours[i], &temps[i], &apparent_temps[i], &precip_probs[i]]
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .max(2); // at least 2 for emoji width

    let print_row = |cells: &[String], emoji: bool| {
        let row: String = cells
            .iter()
            .map(|c| {
                let dw = if emoji { 2 } else { c.len() };
                let padding = col_width.saturating_sub(dw);
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("|{}{}{}", " ".repeat(left_pad), c, " ".repeat(right_pad))
            })
            .collect();
        println!("{}|", row);
    };

    print_row(&hours, false);
    println!("{}", "-".repeat((col_width + 1) * 24 + 1));
    print_row(&emojis, true);
    print_row(&temps, false);
    print_row(&apparent_temps, false);
    print_row(&precip_probs, false);
}

impl Default for TuiPrinter {
    fn default() -> Self {
        Self::new()
    }
}
