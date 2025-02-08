use askama::Template;
use axum::http::header::CACHE_CONTROL;
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::get;

use crate::AppStateRef;
use crate::handlers::pages::IndexPage;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/", get(index))
}

async fn index() -> Response {
    let page = IndexPage {};
    let mut response = Html(page.render().unwrap()).into_response();
    response.headers_mut().insert(
        CACHE_CONTROL,
        "no-cache, no-store".parse().unwrap(),
    );

    response
}
