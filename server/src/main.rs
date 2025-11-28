use generated::server;
use log::info;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use dotenvy::dotenv;

mod handler;
mod service;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let server_impl = handler::ServerImpl::new();

    // 生成されたルーターを使用
    let app = server::new(server_impl).layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    info!("Starting server on http://{}", addr);
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
