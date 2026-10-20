mod greet;

use std::time::Duration;

use axum::{Router, http::Method, routing::get};
use tower_http::{
  cors::{AllowOrigin, CorsLayer},
  trace::TraceLayer,
};

use crate::config::Config;


/// Builds the application router, with every API route under `/api`.
pub fn app(config: &Config) -> Router {
  let api = Router::new()
    .route("/greet", get(greet::greet));

  Router::new()
    .nest("/api", api)
    .layer(cors(config))
    .layer(TraceLayer::new_for_http())
}

/// Configure CORS
fn cors(config: &Config) -> CorsLayer {
  CorsLayer::new()
    .allow_origin(AllowOrigin::list(config.cors_origins
      .iter()
      .cloned()
    ))
    .allow_methods([Method::GET])
    .max_age(Duration::from_secs(600))
}
