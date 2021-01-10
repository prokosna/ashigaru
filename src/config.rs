use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
  pub name: String,
  pub port: String,
  pub beebotte_rest_url: String,
  pub beebotte_mqtt_url: String,
  pub beebotte_channel_token: String,
  pub beebotte_orders_channel: String,
  pub beebotte_events_channel: String,
}
