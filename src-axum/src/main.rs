#[path = "axum.config.rs"]
mod config;
mod routes;

use std::process;

use shared::env::AxumEnv;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::config::Config;


#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "web_book_3d_server=debug,tower_http=debug".into()),
    )
    .init();

  let env = AxumEnv::load().unwrap_or_else(|err| {
    eprintln!("error: {err}");
    process::exit(1);
  });
  let config = Config::from_env(&env);
  let app = routes::app(&config);

  let listener = TcpListener::bind(config.bind_addr)
    .await
    .unwrap_or_else(|err| panic!("failed to bind {}: {err}", config.bind_addr));
  tracing::info!("listening on http://{}", config.bind_addr);

  axum::serve(listener, app).await.expect("server error");
}
