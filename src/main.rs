mod cookies;
mod data;
mod handlers;
mod paths;
mod routes;

use tauri::{WebviewUrl, webview::WebviewWindowBuilder};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let base_dir = paths::resolve_base_dir(app.handle())?;
            paths::init(base_dir);

            let std_listener = std::net::TcpListener::bind("127.0.0.1:0")
                .expect("failed to bind loopback listener");
            let port = std_listener.local_addr()?.port();

            tauri::async_runtime::spawn(async move {
                std_listener.set_nonblocking(true).expect("set_nonblocking");
                let listener = tokio::net::TcpListener::from_std(std_listener)
                    .expect("failed to adopt std listener into tokio");
                axum::serve(listener, routes::build_router())
                    .await
                    .unwrap_or_else(|e| panic!("PANIC: axum server died: {e}"));
            });

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(
                    format!("http://127.0.0.1:{port}/").parse().unwrap(),
                ),
            )
            .title("Mixed Up Monster Hunter")
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
