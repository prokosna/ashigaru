use crate::app::*;
use crate::domain::*;
use crate::infra::*;

pub struct AppContext {
  config_manager: ConfigManagerImpl,
  order_repository: OrderRepositoryBeebotte,
  event_publisher: EventPublisherMqtt,
  order_executor: OrderExecutorShell,
}

impl AppContext {
  pub fn new() -> AppContext {
    let config_manager = ConfigManagerImpl::new();
    let order_repository = OrderRepositoryBeebotte::new(config_manager.get_config());
    let event_publisher = EventPublisherMqtt::new(config_manager.get_config());
    let order_executor = OrderExecutorShell::new();
    AppContext {
      config_manager,
      order_repository,
      event_publisher,
      order_executor,
    }
  }
}

impl UsesConfigManager for AppContext {
  type ConfigManager = ConfigManagerImpl;

  fn uses_config_manager(&self) -> &Self::ConfigManager {
    &self.config_manager
  }
}

impl UsesOrderRepository for AppContext {
  type OrderRepository = OrderRepositoryBeebotte;

  fn uses_order_repository(&self) -> &Self::OrderRepository {
    &self.order_repository
  }
}

impl UsesOrderService for AppContext {
  type OrderService = Self;

  fn uses_order_service(&self) -> &Self::OrderService {
    &self
  }
}

impl UsesEventService for AppContext {
  type EventService = Self;

  fn uses_event_service(&self) -> &Self::EventService {
    &self
  }
}

impl UsesEventPublisher for AppContext {
  type EventPublisher = EventPublisherMqtt;

  fn uses_event_publisher(&self) -> &Self::EventPublisher {
    &self.event_publisher
  }
}

impl UsesOrderExecutor for AppContext {
  type OrderExecutor = OrderExecutorShell;

  fn uses_order_executor(&self) -> &Self::OrderExecutor {
    &self.order_executor
  }
}
