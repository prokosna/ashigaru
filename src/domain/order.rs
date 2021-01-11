use crate::domain::Event;
use anyhow::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Order {
  pub id: String,
  pub name: String,
  pub target: Option<String>,
  pub command: String,
}

impl Order {
  pub fn new(name: String, target: Option<String>, command: String) -> Order {
    Order {
      id: Uuid::new_v4().to_simple().to_string(),
      name,
      target,
      command,
    }
  }
}

pub trait OrderRepository {
  fn get_orders(&self) -> Result<Vec<Order>>;
  fn add_order(&self, order: Order) -> Result<()>;
  fn remove_order_by_id(&self, id: String) -> Result<()>;
}

pub trait UsesOrderRepository {
  type OrderRepository: OrderRepository;
  fn uses_order_repository(&self) -> &Self::OrderRepository;
}

pub trait OrderExecutor {
  fn execute(&self, orders: Vec<Order>, event: Event) -> Result<()>;
}

pub trait UsesOrderExecutor {
  type OrderExecutor: OrderExecutor;
  fn uses_order_executor(&self) -> &Self::OrderExecutor;
}
