mod cookies;
mod data;
mod handlers;
mod routes;

use routes::build_router;

/// Entry point.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap_or_else(|e| panic!("PANIC: Could not bind to port!\n{}", e));

    axum::serve(listener, build_router())
        .await
        .unwrap_or_else(|e| {
            panic!("PANIC: Could not serve the application!\n{}", e)
        });
}
