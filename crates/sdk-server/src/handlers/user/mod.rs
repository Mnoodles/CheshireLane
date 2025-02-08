mod migrate;
mod login;
mod create;

use axum::Router;
use crate::AppStateRef;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .merge(migrate::routes())
        .merge(login::routes())
        .merge(create::routes())
}
