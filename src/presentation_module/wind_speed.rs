#[derive(Debug)]
pub struct WindSpeed {
    pub emoji: &'static str,
    pub description: &'static str,
}

impl WindSpeed {
    pub fn new(wind_speed: f32) -> Self {
        Self {
            emoji: wind_emoji(wind_speed),
            description: wind_description(wind_speed),
        }
    }
}

fn wind_description(speed_kmh: f32) -> &'static str {
    match speed_kmh as u32 {
        0..=1 => "Calm",
        2..=5 => "Light air",
        6..=11 => "Light breeze",
        12..=19 => "Gentle breeze",
        20..=28 => "Moderate breeze",
        29..=38 => "Fresh breeze",
        39..=49 => "Strong breeze",
        50..=61 => "High wind",
        62..=74 => "Gale",
        75..=88 => "Strong gale",
        89..=102 => "Storm",
        103..=117 => "Violent storm",
        _ => "Hurricane",
    }
}

fn wind_emoji(speed_kmh: f32) -> &'static str {
    match speed_kmh as u32 {
        0..=5 => "🌬️",
        6..=28 => "💨",
        29..=61 => "🌀",
        _ => "🌪️",
    }
}
