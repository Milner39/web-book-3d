use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GreetQuery {
  name: String,
}

#[derive(Serialize)]
pub struct GreetResponse {
  message: String,
}

/// `GET /api/greet?name=…`: replaces the template's `greet` Tauri command.
pub async fn greet(Query(query): Query<GreetQuery>) -> Json<GreetResponse> {
  Json(GreetResponse {
    message: format!("Hello, {}! You've been greeted from Axum!", query.name),
  })
}
