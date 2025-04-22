mod cookies;
mod data;
mod handlers;
mod routes;

use crate::routes::build_router;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, build_router()).await.unwrap();
}
