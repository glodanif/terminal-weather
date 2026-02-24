use crate::data_module::WeatherData;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonJson {
    current: CurrentWeatherJson,
    forecast: ForecastJson,
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
    is_day: bool,
}

#[derive(Serialize)]
struct ForecastJson {
    daily: Vec<DailyForecastJson>,
}

#[derive(Serialize)]
struct DailyForecastJson {
    time: String,
    temperature_2m_max: Option<FloatValue>,
    temperature_2m_min: Option<FloatValue>,
    weather_code: Option<u8>,
    hourly: Vec<HourlyForecastJson>,
}

#[derive(Serialize)]
struct HourlyForecastJson {
    time: String,
    temperature: FloatValue,
    apparent_temperature: FloatValue,
    wind_speed: FloatValue,
    precipitation_probability: IntValue,
    cloud_cover: IntValue,
    relative_humidity: IntValue,
    weather_code: u8,
    is_day: bool,
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
    let units = weather_data.current_units;
    let current_weather = weather_data.current;
    let current_weather_json = CurrentWeatherJson {
        temperature: FloatValue {
            value: current_weather.temperature_2m,
            unit: units.temperature_2m,
        },
        apparent_temperature: FloatValue {
            value: current_weather.apparent_temperature,
            unit: units.apparent_temperature,
        },
        wind_speed: FloatValue {
            value: current_weather.wind_speed_10m,
            unit: units.wind_speed_10m,
        },
        precipitation_probability: IntValue {
            value: current_weather.precipitation_probability,
            unit: units.precipitation_probability,
        },
        cloud_cover: IntValue {
            value: current_weather.cloud_cover,
            unit: units.cloud_cover,
        },
        relative_humidity: IntValue {
            value: current_weather.relative_humidity_2m,
            unit: units.relative_humidity_2m,
        },
        weather_code: current_weather.weather_code,
        is_day: current_weather.is_day == 1,
    };

    let daily = weather_data.daily;
    let d = for i in 0..7 {
        let temperature_2m_max = daily.temperature_2m_max[i];
        let tm = match temperature_2m_max {
            None => { None }
            Some(temp) => {
                Some(FloatValue {
                    value: temp,
                    unit: units.temperature_2m.clone(),
                })
            }
        };

        DailyForecastJson {
            time: daily.time[i].clone(),
            temperature_2m_max: daily.temperature_2m_max[i].clone(),
            temperature_2m_min: daily_forecast.temperature_2m_min.clone(),
            weather_code: daily_forecast.weather_code.clone(),
            hourly: daily_forecast.hourly.clone(),
        }
    }


    DailyForecastJson {
        time: daily.time.clone(),
        temperature_2m_max: daily_forecast.temperature_2m_max.clone(),
        temperature_2m_min: daily_forecast.temperature_2m_min.clone(),
        weather_code: daily_forecast.weather_code.clone(),
        hourly: daily_forecast.hourly.clone(),
    }

    JsonJson {
        current: current_weather_json,
    }
}
