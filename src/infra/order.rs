use crate::config;
use crate::domain;
use anyhow::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{self, json};

#[derive(Deserialize)]
pub struct Payload {
  #[serde(rename = "_id")]
  pub id: String,
  pub data: String,
  pub ts: u64,
}

pub struct OrderRepositoryBeebotte {
  config: config::Config,
}

impl OrderRepositoryBeebotte {
  pub fn new(config: config::Config) -> Self {
    OrderRepositoryBeebotte { config }
  }
  fn fetch_oldest_payload(&self) -> Result<Payload> {
    let client = Client::new();

    let res = client
      .get(&format!(
        "{}/{}/{}",
        self.config.beebotte_rest_url, "data/read", self.config.beebotte_orders_channel
      ))
      .header("X-Auth-Token", &self.config.beebotte_channel_token)
      .send()?
      .text()?;

    let mut payloads = serde_json::from_str::<Vec<Payload>>(&res)?;
    payloads.sort_by_key(|x| x.ts);
    payloads.reverse();

    payloads.pop().ok_or(anyhow!("No data in orders channel"))
  }
}

impl domain::OrderRepository for OrderRepositoryBeebotte {
  fn get_orders(&self) -> Result<Vec<domain::Order>> {
    let payload = self.fetch_oldest_payload()?;

    let orders = serde_json::from_str::<Vec<domain::Order>>(&payload.data)?;

    Ok(orders)
  }
  fn add_order(&self, order: domain::Order) -> Result<()> {
    let payload = self.fetch_oldest_payload()?;

    let mut orders = serde_json::from_str::<Vec<domain::Order>>(&payload.data)?;
    orders.push(order);

    let client = Client::new();

    let body = serde_json::to_string(&json!({
      "_id": payload.id,
      "data": serde_json::to_string(&orders)?
    }))?;

    client
      .post(&format!(
        "{}/{}/{}",
        self.config.beebotte_rest_url, "data/update", self.config.beebotte_orders_channel
      ))
      .header("X-Auth-Token", &self.config.beebotte_channel_token)
      .header("Content-Type", "application/json")
      .body(body)
      .send()?;

    Ok(())
  }
  fn remove_order_by_id(&self, id: String) -> Result<()> {
    let payload = self.fetch_oldest_payload()?;

    let mut orders = serde_json::from_str::<Vec<domain::Order>>(&payload.data)?;
    orders.retain(|x| x.id != id);

    let client = Client::new();

    let body = serde_json::to_string(&json!({
      "_id": payload.id,
      "data": serde_json::to_string(&orders)?
    }))?;

    client
      .post(&format!(
        "{}/{}/{}",
        self.config.beebotte_rest_url, "data/update", self.config.beebotte_orders_channel
      ))
      .header("X-Auth-Token", &self.config.beebotte_channel_token)
      .header("Content-Type", "application/json")
      .body(body)
      .send()?;

    Ok(())
  }
}
