use crate::domain::{self, ConfigManager, Event, Order, OrderExecutor, OrderRepository};
use anyhow::*;
use serde_json;
use std::time::Duration;

pub trait EventService:
  domain::UsesOrderRepository + domain::UsesOrderExecutor + domain::UsesConfigManager
{
  fn handle_event(&self, data: &str) -> Result<()> {
    let event = serde_json::from_str::<Event>(data)?;
    let order_repository = self.uses_order_repository();
    let orders = order_repository.get_orders()?;
    let config_manager = self.uses_config_manager();
    let config = config_manager.get_config();

    let matched_orders: Vec<Order> = orders
      .into_iter()
      .filter(|x| x.name == event.order_name)
      .filter(|x| x.target.is_none() || x.clone().target.unwrap_or(String::from("")) == config.name)
      .collect();

    let duration = match event.duration {
      Some(x) => Duration::from_secs(x),
      None => Duration::from_secs(0),
    };

    let order_executor = self.uses_order_executor();
    order_executor.execute(matched_orders, duration)
  }
}

impl<T: domain::UsesOrderRepository + domain::UsesOrderExecutor + domain::UsesConfigManager>
  EventService for T
{
}

pub trait UsesEventService {
  type EventService: EventService;
  fn uses_event_service(&self) -> &Self::EventService;
}
