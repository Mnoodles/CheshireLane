use axum::{Form, Json, Router};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppStateRef;
use common::logging;
use crate::handlers::BaseRsp;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/user/login", post(login))
}

#[derive(Deserialize, Debug)]
struct LoginRequest {
    #[allow(dead_code)]
    uid: u32,
    #[allow(dead_code)]
    captcha_id: Option<String>,
    #[allow(dead_code)]
    gen_time: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "storeId")]
    store_id: String,
    #[allow(dead_code)]
    captcha_output: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "deviceId")]
    device_id: String,
    #[allow(dead_code)]
    pass_token: Option<String>,
    #[allow(dead_code)]
    platform: String,
    #[allow(dead_code)]
    lot_number: Option<String>,
    #[allow(dead_code)]
    token: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    result: i32,
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "birth")]
    birth: Option<Value>,
    transcode: String,
    current_timestamp_ms: i64,
    check7until: i64,
    migrated: bool,
    show_migrate_page: bool,
    #[serde(rename = "channelId")]
    channel_id: String,
    kr_kmc_status: i32,
}

async fn login(
    State(state): State<AppStateRef>,
    request: Form<LoginRequest>,
) -> Response {
    logging::debug!("login: {:?}", request);

    if let Ok(Some(account)) = state.db
        .get_account_by_uid(request.uid.to_string())
        .await {
        let current_timestamp_ms = common::time::now_timestamp_ms();
        let seven_days_ms = 7 * 24 * 60 * 60 * 1000;
        let check7until = current_timestamp_ms + seven_days_ms;

        if account.token == request.token {
            Json(LoginResponse {
                result: 0,
                access_token: account.token,
                birth: None,
                transcode: "NULL".to_string(),
                current_timestamp_ms,
                check7until,
                migrated: false,
                show_migrate_page: false,
                channel_id: request.store_id.clone(),
                kr_kmc_status: 2,
            }).into_response()
        } else {
            Json(BaseRsp { result: 1 }).into_response()
        }
    } else {
        Json(BaseRsp { result: 1 }).into_response()
    }
}
