use std::net::{IpAddr, SocketAddr};

use config::{Config, ConfigError, Environment, File};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub host: SocketAddr,
}

impl Settings {
    pub fn from(config_dir: &str) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        info!("Reading file 'config/default'");
        let default_path = format!("{}/default", config_dir);
        s.merge(File::with_name(&default_path))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        info!("Reading file 'config/local'");
        let local_path = format!("{}/local", config_dir);
        s.merge(File::with_name(&local_path).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        info!("Reading environment");
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
