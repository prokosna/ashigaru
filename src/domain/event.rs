use anyhow::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Eq, Serialize, Debug, Deserialize)]
pub struct Event {
  pub id: String,
  pub order_name: String,
  pub duration: Option<u64>,
  pub created_at: DateTime<Utc>,
}

pub trait EventPublisher {
  fn publish(&self, event: impl Serialize) -> Result<()>;
}

pub trait UsesEventPublisher {
  type EventPublisher: EventPublisher;
  fn uses_event_publisher(&self) -> &Self::EventPublisher;
}
