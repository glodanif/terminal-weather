mod config_storage;
mod data_module_error;
mod location;
mod open_meteo_api_client;
mod weather_data;

use config_storage::ConfigStorage;
use data_module_error::DataModuleError;
pub(crate) use location::Location;
use open_meteo_api_client::OpenMeteoApiClient;
pub(crate) use weather_data::WeatherData;

pub struct DataModule {
    config_storage: ConfigStorage,
    open_meteo_api_client: OpenMeteoApiClient,
}

impl DataModule {
    pub fn new() -> Self {
        DataModule {
            config_storage: ConfigStorage::default(),
            open_meteo_api_client: OpenMeteoApiClient::default(),
        }
    }

    pub fn get_weather_data(&self) -> Result<WeatherData, DataModuleError> {
        let config = self.config_storage.get_config()?;
        let weather_data = self
            .open_meteo_api_client
            .get_weather_data(config.location)?;
        Ok(weather_data)
    }

    pub fn set_location(&self, location: Location) -> Result<(), DataModuleError> {
        self.config_storage.set_location(location)
    }
}

impl Default for DataModule {
    fn default() -> Self {
        Self::new()
    }
}
