#[derive(Debug)]
pub struct Wmo {
    pub emoji: &'static str,
    pub description: &'static str,
    pub nerd_character: &'static str,
}

impl Wmo {
    pub fn new(code: u8, is_day: bool) -> Self {
        Self {
            emoji: wmo_emoji(code, is_day),
            description: wmo_description(code),
            nerd_character: wmo_nerd_character(code, is_day),
        }
    }
}

fn wmo_emoji(code: u8, is_day: bool) -> &'static str {
    match code {
        // Clear sky
        0 => {
            if is_day {
                "🌞" // instead of "☀️"
            } else {
                "🌙"
            }
        }

        // Mainly clear, partly cloudy, overcast
        1 => {
            if is_day {
                "🌤" // without FE0F
            } else {
                "🌙"
            }
        }
        2 => {
            if is_day {
                "⛅"
            } else {
                "🌥" // instead of "☁️"
            }
        }
        3 => "🌥", // instead of "☁️"

        // Fog
        45 | 48 => "🌫", // without FE0F

        // Drizzle
        51 | 53 | 55 => "🌦", // without FE0F
        56 | 57 => "🌨",

        // Rain
        61 | 63 | 65 => "🌧", // without FE0F
        66 | 67 => "🌨",

        // Snow
        71 | 73 | 75 => "❄️",
        77 => "🌨",

        // Rain showers
        80 | 81 | 82 => "🌦",

        // Snow showers
        85 | 86 => "🌨",

        // Thunderstorm
        95 => "⛈", // without FE0F
        96 | 99 => "⛈",

        _ => "🌡", // without FE0F
    }
}

fn wmo_nerd_character(code: u8, is_day: bool) -> &'static str {
    match code {
        // Clear sky
        0 => {
            if is_day {
                "\u{f0599}" // nf-md-weather_sunny
            } else {
                "\u{f0594}" // nf-md-weather_night
            }
        }
        // Mainly clear, partly cloudy
        1 | 2 => {
            if is_day {
                "\u{f0595}" // nf-md-weather_partly_cloudy
            } else {
                "\u{f0f31}" // nf-md-weather_night_partly_cloudy
            }
        }
        // Overcast
        3 => "\u{f0590}", // nf-md-weather_cloudy
        // Fog
        45 | 48 => "\u{f0591}", // nf-md-weather_fog
        // Drizzle
        51 | 53 | 55 => "\u{f0597}", // nf-md-weather_rainy
        56 | 57 => "\u{f0596}",      // nf-md-weather_snowy_rainy
        // Rain
        61 | 63 | 65 => "\u{f0597}", // nf-md-weather_rainy
        66 | 67 => "\u{f0596}",      // nf-md-weather_snowy_rainy
        // Snow
        71 | 73 | 75 | 77 => "\u{f0598}", // nf-md-weather_snowy
        // Rain showers
        80 | 81 | 82 => "\u{f0597}", // nf-md-weather_rainy
        // Snow showers
        85 | 86 => "\u{f0598}", // nf-md-weather_snowy
        // Thunderstorm
        95 | 96 | 99 => "\u{f0593}", // nf-md-weather_lightning
        _ => "\u{f050f}",            // nf-md-weather_cloudy as fallback
    }
}

fn wmo_description(code: u8) -> &'static str {
    match code {
        // Clear / cloudy
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",

        // Fog
        45 => "Fog",
        48 => "Rime fog",

        // Drizzle
        51 => "Light drizzle",
        53 => "Moderate drizzle",
        55 => "Dense drizzle",
        56 => "Light freezing drizzle",
        57 => "Heavy freezing drizzle",

        // Rain
        61 => "Slight rain",
        63 => "Moderate rain",
        65 => "Heavy rain",
        66 => "Light freezing rain",
        67 => "Heavy freezing rain",

        // Snow
        71 => "Slight snowfall",
        73 => "Moderate snowfall",
        75 => "Heavy snowfall",
        77 => "Snow grains",

        // Showers
        80 => "Slight showers",
        81 => "Moderate showers",
        82 => "Violent showers",
        85 => "Slight snow showers",
        86 => "Heavy snow showers",

        // Thunderstorm
        95 => "Thunderstorm",
        96 => "Thunderstorm with slight hail",
        99 => "Thunderstorm with heavy hail",

        _ => "Unknown",
    }
}
