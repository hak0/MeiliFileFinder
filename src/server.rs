use crate::config::MeiliSearchConfig;

use axum::{
    body::Body, extract::{Request, State}, http::uri::Uri, response::{IntoResponse, Response}, routing::any, Router
};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use std::{net::SocketAddr, result::Result};
use tower_http::services::ServeDir;

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

const MEILISEARCH_ENTRY_PREFIX: &str = "/meilisearch";

pub async fn start_server(meilisearch_config: &MeiliSearchConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 在启动服务器前创建index name配置文件
    let frontend_config_path = "static/meilisearch_config.json";
    let config_content = serde_json::json!({
        "index_name": meilisearch_config.meilisearch_index_name
    });
    tokio::fs::write(frontend_config_path, config_content.to_string())
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Serve static files and fallback to index.html for client-side routing
    println!("Starting Server!");
    let file_server = ServeDir::new("static").not_found_service(ServeDir::new("static/index.html"));

    // reverse proxy to meilisearch backend for /api requests
    // and host static files otherwise
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());
    let meilisearch_entry_rule = MEILISEARCH_ENTRY_PREFIX.to_string() + "/{*wildcard}";
    let meilisearch_base_url = meilisearch_config.meilisearch_url.clone();
    let routes= Router::new()
        .route(&meilisearch_entry_rule, any(reverse_proxy_handler))
        .with_state((client, meilisearch_base_url))
        .fallback_service(file_server);

    serve(routes, 3000).await
}

async fn serve(app: Router, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}


async fn reverse_proxy_handler(
    State((client, meilisearch_base_url)): State<(Client, String)>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let path_query_stripped = path_query
        .strip_prefix(MEILISEARCH_ENTRY_PREFIX)
        .unwrap_or(path_query);

    let uri = format!("{}{}", meilisearch_base_url, path_query_stripped);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
