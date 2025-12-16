use crate::handlers::{
    actions::route_actions, components::route_components, pages::route_pages,
};
use axum::Router;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};

const PATH_STATIC_FILES: &str = "static";
const ROUTE_ASSETS: &str = "/assets";

/// Build the main router.
pub fn build_router() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let static_path = std::env::current_dir()
        .unwrap_or_default()
        .join(PATH_STATIC_FILES);

    Router::new()
        .nest_service(ROUTE_ASSETS, ServeDir::new(static_path))
        .merge(route_pages())
        .nest("/_components", route_components())
        .nest("/_actions", route_actions())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CookieManagerLayer::new()),
        )
}
