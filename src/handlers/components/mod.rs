pub mod hunter;
pub mod modals;
pub mod monster;

use crate::handlers::components::{hunter::hunter, monster::monster};
use axum::{Router, routing::get};
use modals::route_modals;

pub fn route_components() -> Router {
    Router::new()
        .route("/hunter/{number}", get(hunter))
        .route("/monster", get(monster))
        .nest("/_modals", route_modals())
}
