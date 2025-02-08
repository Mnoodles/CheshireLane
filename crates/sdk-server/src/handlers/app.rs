use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::{Form, Router};
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;
use tokio::fs;

use common::logging;
use config::CONFIG;
use crate::AppStateRef;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/app/getCode", post(get_code))
        .route("/app/getSettings", post(get_settings))
}

#[derive(Deserialize, Debug)]
struct GetCodeReq {
    #[allow(dead_code)]
    all: String,
    #[allow(dead_code)]
    codestr: String,
}

async fn get_code(request: Form<GetCodeReq>) -> Result<Response, StatusCode> {
    // all:     1
    // codestr: 0
    logging::debug!("getCode: {:?}",request);
    if let Ok(body) = fs::read(CONFIG.sdk_config.code_path.clone())
        .await {
        Ok(([(CONTENT_TYPE, "application/json")], body).into_response())
    } else {
        logging::error!("JSON for getCode not found");
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize, Debug)]
struct GetSettingsReq {
    #[allow(dead_code)]
    #[serde(rename = "storeId")]
    store_id: String,
}

async fn get_settings(request: Form<GetSettingsReq>) -> Result<Response, StatusCode> {
    // storeId: googleplay
    logging::debug!("getSettings: {:?}", request);
    if let Ok(body) = fs::read(CONFIG.sdk_config.settings_path.clone())
        .await {
        Ok(([(CONTENT_TYPE, "application/json")], body).into_response())
    } else {
        logging::error!("JSON for getSettings not found");
        Err(StatusCode::NOT_FOUND)
    }
}
