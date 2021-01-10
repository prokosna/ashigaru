use crate::app::*;
use crate::context::AppContext;
use crate::domain::*;
use actix_web::{
  delete, dev::HttpResponseBuilder, get, http::header, post, web, HttpResponse, Responder,
};
use anyhow::*;
use serde::Deserialize;
use serde_json::json;
use std::fmt::Display;

#[get("/orders")]
pub async fn get_orders(app: web::Data<AppContext>) -> impl Responder {
  let order_service = app.uses_order_service();
  let orders = order_service.get_all_orders();
  orders
    .map(|x| HttpResponse::Ok().json(x))
    .map_err(ApiError::from)
}

#[derive(Deserialize)]
pub struct PostOrderParam {
  name: String,
  target: Option<String>,
  command: String,
}

#[post("/orders")]
pub async fn post_order(
  app: web::Data<AppContext>,
  param: web::Json<PostOrderParam>,
) -> impl Responder {
  let order_service = app.uses_order_service();
  let command = RegisterNewOrderCommand {
    name: param.name.clone(),
    target: param.target.clone(),
    command: param.command.clone(),
  };
  let id = order_service.register_new_order(command);
  id.map(|x| HttpResponse::Ok().json(json!({ "id": x })))
    .map_err(ApiError::from)
}

#[delete("/orders/{id}")]
pub async fn delete_order(
  app: web::Data<AppContext>,
  web::Path(id): web::Path<String>,
) -> impl Responder {
  let order_service = app.uses_order_service();
  order_service
    .remove_order(id)
    .map(|_| HttpResponse::Ok())
    .map_err(ApiError::from)
}

#[get("/status")]
pub async fn get_status(app: web::Data<AppContext>) -> impl Responder {
  let config_manager = app.uses_config_manager();
  let config = config_manager.get_config();
  HttpResponse::Ok().json(config)
}

#[derive(Debug)]
struct ApiError {
  err: Error,
}

impl Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.err)
  }
}

impl actix_web::error::ResponseError for ApiError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
      .body(format!("{}", self.err))
  }
}

impl From<Error> for ApiError {
  fn from(err: Error) -> ApiError {
    ApiError { err }
  }
}
