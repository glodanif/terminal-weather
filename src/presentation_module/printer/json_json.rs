use crate::data_module::WeatherData;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonJson {
    current: CurrentWeatherJson,
}

#[derive(Serialize)]
struct CurrentWeatherJson {
    temperature: FloatValue,
    apparent_temperature: FloatValue,
    wind_speed: FloatValue,
    precipitation_probability: IntValue,
    cloud_cover: IntValue,
    relative_humidity: IntValue,
    weather_code: u8,
    pub is_day: bool,
}

#[derive(Serialize)]
struct FloatValue {
    value: f32,
    unit: String,
}

#[derive(Serialize)]
struct IntValue {
    value: u8,
    unit: String,
}

pub fn create_json_json_response(weather_data: WeatherData) -> JsonJson {
    let current_weather = weather_data.current;
    let current_weather_units = weather_data.current_units;
    let current_weather_json = CurrentWeatherJson {
        temperature: FloatValue {
            value: current_weather.temperature_2m,
            unit: current_weather_units.temperature_2m,
        },
        apparent_temperature: FloatValue {
            value: current_weather.apparent_temperature,
            unit: current_weather_units.apparent_temperature,
        },
        wind_speed: FloatValue {
            value: current_weather.wind_speed_10m,
            unit: current_weather_units.wind_speed_10m,
        },
        precipitation_probability: IntValue {
            value: current_weather.precipitation_probability,
            unit: current_weather_units.precipitation_probability,
        },
        cloud_cover: IntValue {
            value: current_weather.cloud_cover,
            unit: current_weather_units.cloud_cover,
        },
        relative_humidity: IntValue {
            value: current_weather.relative_humidity_2m,
            unit: current_weather_units.relative_humidity_2m,
        },
        weather_code: current_weather.weather_code,
        is_day: current_weather.is_day == 1,
    };
    JsonJson {
        current: current_weather_json,
    }
}
