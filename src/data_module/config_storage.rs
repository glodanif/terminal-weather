use crate::data_module::data_module_error::DataModuleError;
use crate::data_module::location::Location;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub location: Location,
}

const CONFIG_FILE_NAME: &str = "config.toml";

pub struct ConfigStorage {
    config_dir: PathBuf,
    config_file: PathBuf,
}

impl ConfigStorage {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join(env!("CARGO_PKG_NAME"));
        let config_file = config_dir.join(CONFIG_FILE_NAME);
        ConfigStorage {
            config_dir,
            config_file,
        }
    }

    pub fn get_config(&self) -> Result<Config, DataModuleError> {
        if self.config_file.exists() {
            let config_file_content = fs::read_to_string(&self.config_file)
                .map_err(|_| DataModuleError::FailedToGetConfig)?;
            let config: Config = toml::from_str(&config_file_content)
                .map_err(|_| DataModuleError::FailedToGetConfig)?;
            Ok(config)
        } else {
            Err(DataModuleError::ConfigFileNotFound)
        }
    }

    pub fn set_location(&self, location: Location) -> Result<(), DataModuleError> {
        let config = match self.get_config() {
            Ok(mut config) => {
                config.location = location;
                config
            }
            Err(DataModuleError::ConfigFileNotFound) => {
                fs::create_dir_all(&self.config_dir)
                    .map_err(|_| DataModuleError::FailedToSetConfig)?;
                Config { location }
            }
            Err(e) => return Err(e),
        };
        let config_file_content =
            toml::to_string(&config).map_err(|_| DataModuleError::FailedToSetConfig)?;
        fs::write(&self.config_file, config_file_content)
            .map_err(|_| DataModuleError::FailedToSetConfig)?;
        Ok(())
    }
}

impl Default for ConfigStorage {
    fn default() -> Self {
        Self::new()
    }
}
