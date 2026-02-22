#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DataModuleError {
    #[error("Failed to get weather data")]
    FailedToGetWeatherData,
    #[error("Failed to set location")]
    FailedToSetLocation,
    #[error("Location not set")]
    LocationNotSet,
    #[error("Failed to get config")]
    FailedToGetConfig,
    #[error("Failed to set config")]
    FailedToSetConfig,
    #[error("Location not set, use `terminal-weather set-location <latitude> <longitude>` to set it")]
    ConfigFileNotFound,  
    #[error("Config file invalid")]
    ConfigFileInvalid,
}
