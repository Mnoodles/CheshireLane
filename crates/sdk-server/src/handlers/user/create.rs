use axum::extract::State;
use axum::{Form, Json, Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use common::logging;
use database::schema::CheshireDBSchemaGetU32Id;
use crate::AppStateRef;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/user/create", post(create))
}

#[derive(Deserialize, Debug)]
struct CreateRequest {
    #[allow(dead_code)]
    #[serde(rename = "channelId")]
    channel_id: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "deviceId")]
    device_id: String,
}

#[derive(Serialize)]
struct CreateResponse {
    result: i32,
    uid: u32,
    token: String,
    #[serde(rename = "isNew")]
    is_new: i32,
}

async fn create(
    State(state): State<AppStateRef>,
    request: Form<CreateRequest>,
) -> Result<Response, StatusCode> {
    logging::debug!("create: {:?}", request);

    if let Ok(Some(account)) = state.db
        .get_account_by_device_id(request.device_id.clone())
        .await {
        if let Ok(uid) = account.uid() {
            Ok(Json(CreateResponse {
                result: 0,
                uid,
                token: account.token,
                is_new: 0,
            }).into_response())
        } else { Err(StatusCode::INTERNAL_SERVER_ERROR) }
    } else {
        if let Ok(Some(account)) = state.db
            .create_account(request.device_id.clone())
            .await {
            if let Ok(uid) = account.uid() {
                Ok(Json(CreateResponse {
                    result: 0,
                    uid,
                    token: account.token,
                    is_new: 1,
                }).into_response())
            } else { Err(StatusCode::INTERNAL_SERVER_ERROR) }
        } else { Err(StatusCode::INTERNAL_SERVER_ERROR) }
    }
}
