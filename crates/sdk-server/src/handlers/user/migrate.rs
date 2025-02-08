use axum::{Json, Router};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{options, post};
use serde::Deserialize;
use tokio::fs;
use common::logging;
use config::CONFIG;
use crate::AppStateRef;
use crate::handlers::BaseRsp;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/user/migrate/errorcode", post(error_code))
        .route("/user/migrate/none", post(none))
        .route("/user/migrate/errorcode", options(|| async {
            // return 204
            StatusCode::NO_CONTENT
        }))
        .route("/user/migrate/none", options(|| async {
            // return 204
            StatusCode::NO_CONTENT
        }))
}

#[derive(Deserialize, Debug)]
struct ErrorCodeReq {
    #[allow(dead_code)]
    lang: String,
}

async fn error_code(request: Json<ErrorCodeReq>) -> Result<Response, StatusCode> {
    // {
    //     "lang": "en"
    // }
    logging::debug!("errorcode: {:?}", request);
    if let Ok(body) = fs::read(CONFIG.sdk_config.error_code_path.clone())
        .await {
        Ok(([(CONTENT_TYPE, "application/json")], body).into_response())
    } else {
        logging::error!("JSON for errorCode not found");
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize, Debug)]
struct NoneReq {
    #[allow(dead_code)]
    #[serde(rename = "accessToken")]
    access_token: String,
    #[allow(dead_code)]
    check7day: String,
}

async fn none(request: Json<NoneReq>) -> impl IntoResponse {
    // {
    //     "accessToken": "00cdd954e10b459a24448734a6cc62c0",
    //     "check7day": "0"
    // }
    logging::debug!("none: {:?}", request);
    Json(BaseRsp { result: 0 })
}
