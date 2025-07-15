pub mod index;

use crate::handlers::pages::index::index;
use axum::{Router, routing::get};

pub fn route_pages() -> Router {
    Router::new().route("/", get(index))
}
