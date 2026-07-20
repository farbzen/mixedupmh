pub mod hunter;
pub mod monster;

use crate::handlers::components::{hunter::hunter, monster::monster};
use axum::{Router, routing::get};

pub fn route_components() -> Router {
    Router::new()
        .route("/hunter/{number}", get(hunter))
        .route("/monster", get(monster))
}
