use crate::config;

pub trait ConfigManager {
  fn get_config(&self) -> config::Config;
}

pub trait UsesConfigManager {
  type ConfigManager: ConfigManager;
  fn uses_config_manager(&self) -> &Self::ConfigManager;
}
