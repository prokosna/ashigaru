use crate::domain::{self, OrderRepository};
use anyhow::*;

#[derive(Clone)]
pub struct RegisterNewOrderCommand {
  pub name: String,
  pub target: Option<String>,
  pub command: String,
}

pub trait OrderService: domain::UsesOrderRepository {
  fn get_all_orders(&self) -> Result<Vec<domain::Order>> {
    let repo = self.uses_order_repository();
    repo.get_orders()
  }

  fn register_new_order(&self, cmd: RegisterNewOrderCommand) -> Result<String> {
    let repo = self.uses_order_repository();

    let order = domain::Order::new(cmd.name, cmd.target, cmd.command);
    let id = order.id.clone();

    repo.add_order(order)?;

    Ok(id)
  }

  fn remove_order(&self, id: String) -> Result<()> {
    let repo = self.uses_order_repository();

    repo.remove_order_by_id(id)?;
    Ok(())
  }
}

impl<T: domain::UsesOrderRepository> OrderService for T {}

pub trait UsesOrderService {
  type OrderService: OrderService;
  fn uses_order_service(&self) -> &Self::OrderService;
}
