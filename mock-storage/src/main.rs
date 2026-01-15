use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Path},
    http::StatusCode,
    response::Response,
    routing::put,
    Router,
};
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use dotenvy::dotenv;

/// ファイルをアップロードするエンドポイント
/// PUT /upload/{filename}
async fn upload_file(
    Path(filename): Path<String>,
    body: Bytes,
) -> Result<Response<String>, StatusCode> {
    info!("Received upload request for file: {}", filename);

    if body.is_empty() {
        error!("Empty file body received");
        return Err(StatusCode::BAD_REQUEST);
    }

    // 保存先ディレクトリを取得
    let storage_dir = env::var("MOCK_STORAGE_DIR")
        .unwrap_or_else(|_| "./mock-storage".to_string());
    let storage_path = PathBuf::from(&storage_dir);

    // ディレクトリが存在しない場合は作成
    if !storage_path.exists() {
        info!("Creating storage directory: {}", storage_dir);
        fs::create_dir_all(&storage_path)
            .await
            .map_err(|e| {
                error!("Failed to create storage directory: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    // ファイルパスを構築
    let file_path = storage_path.join(&filename);

    // ファイルを保存
    info!(
        "Saving file: {} (size: {} bytes) to {}",
        filename,
        body.len(),
        file_path.display()
    );
    fs::write(&file_path, &body)
        .await
        .map_err(|e| {
            error!("Failed to write file {}: {}", file_path.display(), e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("File saved successfully: {}", file_path.display());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("File uploaded successfully".to_string())
        .unwrap())
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();

    // ポート番号を取得（デフォルト: 9000）
    let port = env::var("MOCK_STORAGE_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .expect("MOCK_STORAGE_PORT must be a valid port number");

    // 保存先ディレクトリを取得
    let storage_dir = env::var("MOCK_STORAGE_DIR")
        .unwrap_or_else(|_| "./mock-storage".to_string());

    info!("Starting mock storage server");
    info!("Port: {}", port);
    info!("Storage directory: {}", storage_dir);

    // ボディサイズ制限を緩和（デフォルトは2MB、100MBに設定）
    // 環境変数で設定可能（デフォルト: 100MB = 100 * 1024 * 1024 bytes）
    let body_limit = env::var("MOCK_STORAGE_BODY_LIMIT")
        .unwrap_or_else(|_| "104857600".to_string()) // 100MB
        .parse::<usize>()
        .unwrap_or(104857600);
    info!("Body size limit: {} bytes ({} MB)", body_limit, body_limit / 1024 / 1024);

    // ルーターを構築
    let app = Router::new()
        .route("/upload/{filename}", put(upload_file))
        .layer(DefaultBodyLimit::max(body_limit));

    // サーバーを起動
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Mock storage server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
