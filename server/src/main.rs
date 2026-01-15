use dotenvy::dotenv;
use generated::server;
use log::info;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod handler;
mod infrastructure;
mod mock;
mod service;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();

    let converter = Arc::new(infrastructure::DefaultConverter::new());
    let storage = Arc::new(infrastructure::DefaultStorage::new());
    let upload_service = Arc::new(service::UploadSingleImageServiceImpl::new(
        converter.clone(),
        storage.clone(),
    ));
    let upload_merged_service = Arc::new(service::UploadMergedImageServiceImpl::new(
        converter,
        storage,
    ));
    let server_impl = handler::ServerImpl::new(upload_service, upload_merged_service);

    // 生成されたルーターを使用
    let app = server::new(server_impl).layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    let host = env::var("API_SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("API_SERVER_PORT").unwrap_or_else(|_| "9090".to_string());
    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid host or port for API_SERVER_HOST or API_SERVER_PORT");
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
