mod handlers;

use std::net::SocketAddr;
use std::sync::OnceLock;
use anyhow::{anyhow, Result};
use axum::Router;
use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::ServeDir;
use config::CONFIG;
use common::{banner, show_info, logging};
use database::DbContext;

struct AppState {
    pub db: DbContext,
}

type AppStateRef = &'static AppState;

#[derive(thiserror::Error, Debug)]
enum SDKServerError {
    #[error("Failed to install rustls crypto provider")]
    CryptoProviderCreationFailure,
    #[error("Start server error: {0}")]
    StartServerFailure(#[from] std::io::Error),
}

// Fake server for host `azurusapi.yo-star.com`

#[tokio::main]
async fn main() -> Result<()> {
    banner();

    static STATE: OnceLock<AppState> = OnceLock::new();
    logging::init(logging::Level::DEBUG);
    rustls::crypto::ring::default_provider().install_default()
        .map_err(|_| anyhow!(SDKServerError::CryptoProviderCreationFailure))?;

    show_info();

    let db = DbContext::connect().await?;
    let state = STATE.get_or_init(move || AppState { db });

    let static_files = ServeDir::new("assets/static");

    let handle = Handle::new();
    let tls = RustlsConfig::from_pem_file(
        CONFIG.tls_config.cert_path.clone(),
        CONFIG.tls_config.key_path.clone()).await?;
    let app = Router::new()
        .merge(handlers::routes())
        .with_state(state)
        .nest_service("/static", static_files);

    let http_addr: SocketAddr = CONFIG.sdk_config.http_addr.parse()?;
    let https_addr: SocketAddr = CONFIG.sdk_config.https_addr.parse()?;

    let app_clone = app.clone();
    let handle_clone = handle.clone();
    let http = tokio::spawn(async move {
        logging::info!("HTTP server listening on {}", http_addr);
        axum_server::bind(http_addr)
            .handle(handle_clone)
            .serve(app_clone.into_make_service())
            .await
            .map_err(|e| SDKServerError::StartServerFailure(e))
    });
    let https = tokio::spawn(async move {
        logging::info!("HTTPS server listening on {}", https_addr);
        axum_server::bind_rustls(https_addr, tls)
            .handle(handle)
            .serve(app.into_make_service())
            .await
            .map_err(|e| SDKServerError::StartServerFailure(e))
    });

    http.await??;
    https.await??;

    Ok(())
}
