use std::net::SocketAddr;

use axum::http::HeaderValue;
use shared::env::AxumEnv;


/// Server settings, built from the env vars in `AxumEnv`
pub struct Config {
  /// Address the server listens on
  pub bind_addr: SocketAddr,

  /// Origins allowed to call the API cross-origin
  pub cors_origins: Vec<HeaderValue>,
}

impl Config {
  pub fn from_env(env: &AxumEnv) -> Self {
    // `AXUM_CORS_ORIGINS`, plus `TAURI_DEV_URL` when it's set (only in dev)
    let cors_origins = env
      .cors_origins
      .iter()
      .chain(&env.tauri_dev_url)
      .map(|origin| {
        HeaderValue::from_str(origin.as_str())
          .expect("an Origin only holds visible ASCII")
      })
      .collect();

    Self {
      bind_addr: env.listen_bind,
      cors_origins,
    }
  }
}
