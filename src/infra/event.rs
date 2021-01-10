use crate::app::*;
use crate::config;
use crate::context;
use crate::domain;
use actix_web::web;
use anyhow::*;
use log::*;
use paho_mqtt as mqtt;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct Payload {
  pub data: String,
  pub ispublic: bool,
  pub ts: u64,
}

pub struct EventPublisherMqtt {
  config: config::Config,
}

impl EventPublisherMqtt {
  pub fn new(config: config::Config) -> Self {
    EventPublisherMqtt { config }
  }
}

impl domain::EventPublisher for EventPublisherMqtt {
  fn publish(&self, event: impl Serialize) -> Result<()> {
    let client = mqtt::Client::new(self.config.beebotte_mqtt_url.clone())?;

    let options = mqtt::ConnectOptionsBuilder::new()
      .keep_alive_interval(Duration::from_secs(20))
      .clean_session(true)
      .user_name(self.config.beebotte_channel_token.clone())
      .finalize();

    client.connect(options)?;

    let payload = json!({
      "data": to_string(&event)?,
      "write": false,
    });

    let msg = mqtt::MessageBuilder::new()
      .topic(self.config.beebotte_events_channel.clone())
      .payload(to_string(&payload)?)
      .qos(1)
      .finalize();

    client.publish(msg)?;

    client.disconnect(None).map_err(|e| anyhow!("{}", e))
  }
}

pub struct EventReceiverMqtt {
  config: config::Config,
}

impl EventReceiverMqtt {
  pub fn new(config: config::Config) -> Self {
    EventReceiverMqtt { config }
  }

  fn reconnect(&self, client: &mqtt::Client) -> bool {
    warn!("MQTT connection lost. Waiting to retry connection");
    for _ in 0..12 {
      thread::sleep(Duration::from_millis(5000));
      if client.reconnect().is_ok() {
        info!("MQTT successfully reconnected");
        return true;
      }
    }
    error!("Unable to reconnect MQTT after several attempts");
    false
  }

  pub fn wait_for_events(&self, app: web::Data<context::AppContext>) -> Result<()> {
    let create_options = mqtt::CreateOptionsBuilder::new()
      .server_uri(self.config.beebotte_mqtt_url.clone())
      .client_id(self.config.name.clone())
      .finalize();

    let mut client = mqtt::Client::new(create_options)?;

    let rx = client.start_consuming();

    let con_options = mqtt::ConnectOptionsBuilder::new()
      .keep_alive_interval(Duration::from_secs(20))
      .clean_session(false)
      .user_name(self.config.beebotte_channel_token.clone())
      .finalize();

    let resp = client.connect(con_options)?;
    if let Some(conn_resp) = resp.connect_response() {
      if !conn_resp.session_present {
        client.subscribe(&self.config.beebotte_events_channel, 1)?;
      }
    }

    for msg in rx.iter() {
      if let Some(msg) = msg {
        match serde_json::from_str::<Payload>(&msg.payload_str()) {
          Ok(payload) => {
            let app1 = app.clone();
            thread::spawn(move || {
              let event_service = app1.get_ref().uses_event_service();
              if let Err(e) = event_service.handle_event(&payload.data) {
                error!("Failed to handle event: {}", e);
              }
            });
          }
          Err(_) => {
            info!("Ignore invalid payload: {}", msg);
          }
        }
      } else if client.is_connected() || !self.reconnect(&client) {
        break;
      }
    }

    if client.is_connected() {
      client.unsubscribe(&self.config.beebotte_events_channel)?;
      client.disconnect(None).map_err(|e| anyhow!("{}", e))?;
    }

    Ok(())
  }
}
