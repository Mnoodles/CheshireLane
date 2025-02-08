use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{post, options};
use serde::Deserialize;

use common::logging;
use crate::AppStateRef;
use crate::handlers::BaseRsp;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/account/yostar_auth_request", post(yostar_auth_request))
        .route("/account/yostar_auth_submit", post(yostar_auth_submit))
        .route("/account/yostar_auth_request", options(|| async {
            // return 204
            StatusCode::NO_CONTENT
        }))
        .route("/account/yostar_auth_submit", options(|| async {
            // return 204
            StatusCode::NO_CONTENT
        }))
}

#[derive(Deserialize, Debug)]
struct YostarAuthRequestReq {
    #[allow(dead_code)]
    account: String,
    #[allow(dead_code)]
    authlang: String,
}

async fn yostar_auth_request(request: Json<YostarAuthRequestReq>) -> impl IntoResponse {
    // {
    //     "account": "aaa@aaa.com",
    //     "authlang": "en"
    // }
    logging::debug!("yostar_auth_request: {:?}", request);
    Json(BaseRsp { result: 0 })
}

#[derive(Deserialize, Debug)]
struct YostarAuthSubmitReq {
    #[allow(dead_code)]
    account: String,
    #[allow(dead_code)]
    code: String,
    #[allow(dead_code)]
    authlang: String,
}

async fn yostar_auth_submit(request: Json<YostarAuthSubmitReq>) -> impl IntoResponse {
    // {
    //     "account": "aaa@aaa.com",
    //     "code": "0000",
    //     "authlang": "en"
    // }
    logging::debug!("yostar_auth_submit: {:?}", request);
    Json(BaseRsp { result: 0 })
}
