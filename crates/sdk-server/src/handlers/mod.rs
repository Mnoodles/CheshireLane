mod pages;
mod app;
mod index;
mod account;
mod user;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use serde::Serialize;
use common::logging;
use crate::AppStateRef;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .merge(app::routes())
        .merge(index::routes())
        .merge(account::routes())
        .merge(user::routes())
        .fallback(fallback_handle)
}

#[derive(Serialize)]
pub struct BaseRsp {
    pub result: i32,
}

async fn fallback_handle(uri: axum::http::Uri) -> Response {
    let message = format!("NOT_FOUND: The requested path '{}' was not found.", uri);
    logging::warn!("{}", message);
    (StatusCode::NOT_FOUND, message).into_response()
}
