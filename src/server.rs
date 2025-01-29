use axum::{routing::get, Router};
use std::{net::SocketAddr, result::Result};
use tower_http::services::ServeDir;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // Serve static files and fallback to index.html for client-side routing
    println!("Starting Server!");
    let serve_dir = ServeDir::new("static").not_found_service(ServeDir::new("assets/index.html"));

    let app = Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .fallback_service(serve_dir);

    serve(app, 3000).await
}

async fn serve(app: Router, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
