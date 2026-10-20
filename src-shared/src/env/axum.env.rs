use std::net::SocketAddr;

use crate::{env, types::Origin};


env::schema! {
  /// Every env var the Axum server reads
  pub struct AxumEnv {
    /// Address the server listens on
    pub listen_bind: SocketAddr = env::parse("AXUM_LISTEN_BIND").required(),


    /// Allowed CORS origins,
    /// e.g. `tauri://localhost, http://tauri.localhost` in production
    pub cors_origins: Vec<Origin> = env::list("AXUM_CORS_ORIGINS").default(Vec::new()),

    /// The Tauri WebView's dev origin, allowed for CORS (dev only)
    pub tauri_dev_url: Option<Origin> = env::parse("TAURI_DEV_URL").optional(),
  }
}
