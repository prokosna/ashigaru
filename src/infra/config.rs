use crate::config;
use crate::domain;
use envy;
use log::*;

pub struct ConfigManagerImpl {
  config: config::Config,
}

impl ConfigManagerImpl {
  pub fn new() -> Self {
    let config = match envy::prefixed("ASGR_").from_env::<config::Config>() {
      Ok(val) => val,
      Err(err) => {
        error!("{}", err);
        panic!("Failed to load config");
      }
    };

    ConfigManagerImpl { config }
  }
}

impl domain::ConfigManager for ConfigManagerImpl {
  fn get_config(&self) -> config::Config {
    self.config.clone()
  }
}
