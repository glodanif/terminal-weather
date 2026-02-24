use crate::data_module::data_module_error::DataModuleError;
use crate::data_module::location::Location;
use crate::data_module::weather_data::WeatherData;

const OPEN_METEO_API_URL: &str = "https://api.open-meteo.com/v1/forecast";
const OPEN_METEO_API_HOURLY_VALUES: &str = "temperature_2m,wind_speed_10m,apparent_temperature,weather_code,cloud_cover,precipitation_probability,is_day,relative_humidity_2m";
const OPEN_METEO_API_DAILY_VALUES: &str = "temperature_2m_max,temperature_2m_min,weather_code";

pub struct OpenMeteoApiClient {}

impl OpenMeteoApiClient {
    pub fn new() -> Self {
        OpenMeteoApiClient {}
    }

    pub fn get_weather_data(&self, location: Location) -> Result<WeatherData, DataModuleError> {
        let url = format!(
            "{}?latitude={}&longitude={}&current={}&daily={}&hourly={}&forecast_days=7&timezone=auto",
            OPEN_METEO_API_URL,
            location.latitude,
            location.longitude,
            OPEN_METEO_API_HOURLY_VALUES,
            OPEN_METEO_API_DAILY_VALUES,
            OPEN_METEO_API_HOURLY_VALUES
        );
        let response =
            reqwest::blocking::get(url).map_err(|_| DataModuleError::FailedToGetWeatherData)?;

        if response.status().is_success() {
            let response_json = response
                .json()
                .map_err(|_| DataModuleError::FailedToGetWeatherData)?;
            let open_meteo_data: WeatherData = serde_json::from_value(response_json)
                .map_err(|_| DataModuleError::FailedToGetWeatherData)?;
            Ok(open_meteo_data)
        } else {
            Err(DataModuleError::FailedToGetWeatherData)
        }
    }
}

impl Default for OpenMeteoApiClient {
    fn default() -> Self {
        Self::new()
    }
}
