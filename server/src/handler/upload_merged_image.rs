use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use generated::models;
use http::Method;
use log::info;

/// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
pub async fn handle(
    _method: &Method,
    _host: &Host,
    _cookies: &CookieJar,
    mut body: Multipart,
) -> Result<apis::default::UploadMergedImageResponse, ()> {
    info!("upload_merged_image() called");

    let mut signed_url: Option<String> = None;
    let mut metadata: Option<String> = None;
    let mut _files: Vec<Vec<u8>> = Vec::new();

    while let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if let Ok(data) = field.bytes().await {
            match name.as_str() {
                "signedUrl" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        signed_url = Some(s);
                    }
                }
                "metadata" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        metadata = Some(s);
                    }
                }
                "files" => {
                    _files.push(data.to_vec());
                }
                _ => {
                    log::warn!("Unknown field: {}", name);
                }
            }
        }
    }

    if signed_url.is_none() || metadata.is_none() {
        return Ok(
            apis::default::UploadMergedImageResponse::Status400_BadRequest(models::ErrorResponse {
                message: "Bad Request".to_string(),
                error_code: "INVALID_INPUT".to_string(),
                details: None,
            }),
        );
    }

    // TODO: 実装を追加
    // - 複数画像をDDS形式に変換
    // - 1ファイルにまとめる
    // - ストレージにアップロード

    Ok(
        apis::default::UploadMergedImageResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: "success".to_string(),
                data: None,
            },
        ),
    )
}
