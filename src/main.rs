pub mod app;
pub mod config;
pub mod context;
pub mod domain;
pub mod infra;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use domain::*;
use dotenv::dotenv;
use env_logger;
use log::*;
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let context = context::AppContext::new();
  let app = web::Data::new(context);
  let config_manager = app.uses_config_manager();

  // Run MQTT receiver
  let event_receiver = infra::EventReceiverMqtt::new(config_manager.get_config());
  let app1 = app.clone();
  thread::spawn(move || {
    if let Err(e) = event_receiver.wait_for_events(app1) {
      error!("{}", e);
    }
    panic!("MQTT receiver thread was terminated");
  });

  // Run HTTP server
  let config = config_manager.get_config();
  HttpServer::new(move || {
    App::new()
      .app_data(app.clone())
      .service(infra::get_status)
      .service(infra::get_orders)
      .service(infra::post_order)
      .service(infra::delete_order)
      .service(fs::Files::new("/", "./static").index_file("index.html"))
  })
  .bind(format!("0.0.0.0:{}", config.port))?
  .run()
  .await
}
