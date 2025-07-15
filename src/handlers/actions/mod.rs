pub mod hunter;
pub mod language;

use crate::handlers::actions::{
    hunter::{hunter_count, rename_hunter},
    language::switch_language,
};
use axum::{Router, routing::get};

pub fn route_actions() -> Router {
    Router::new()
        .route("/hunter/rename", get(rename_hunter))
        .route("/hunter/count", get(hunter_count))
        .route("/language/{language}", get(switch_language))
}
