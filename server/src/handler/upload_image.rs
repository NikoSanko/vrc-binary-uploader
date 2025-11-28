use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use generated::models;
use http::Method;
use log::{info, warn};

use crate::service::{UploadSingleImageService, ServiceError};

/// １枚の画像をDDS形式に変換し、ストレージにアップロードする
pub async fn handle(
    _method: &Method,
    _host: &Host,
    _cookies: &CookieJar,
    mut body: Multipart,
    service: &dyn UploadSingleImageService,
) -> Result<apis::default::UploadImageResponse, ()> {
    info!("upload_image() called");

    let mut signed_url: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        info!("name: {}", name);
        if let Ok(data) = field.bytes().await {
            match name.as_str() {
                "signedUrl" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        signed_url = Some(s);
                    }
                }
                "file" => {
                    info!("file: {}", data.len());
                    file_data = Some(data.to_vec());
                }
                _ => {
                    warn!("Unknown field: {}", name);
                }
            }
        } else {
            warn!("Failed to parse body to bytes");
        }
    }

    if signed_url.is_none() || file_data.is_none() {
        return Ok(apis::default::UploadImageResponse::Status400_BadRequest(
            models::ErrorResponse {
                message: "Bad Request".to_string(),
                error_code: "INVALID_INPUT".to_string(),
                details: None,
            },
        ));
    }

    let signed_url = signed_url.unwrap();
    let file_data = file_data.unwrap();

    // NOTE: 実処理
    match service.execute(&signed_url, &file_data).await {
        Ok(_) => {}
        Err(ServiceError::Validation(msg)) => {
            return Ok(apis::default::UploadImageResponse::Status400_BadRequest(
                models::ErrorResponse {
                    message: msg,
                    error_code: "INVALID_INPUT".to_string(),
                    details: None,
                },
            ));
        }
        Err(ServiceError::Infrastructure(e)) => {
            log::error!("Infrastructure error: {}", e);
            return Ok(apis::default::UploadImageResponse::Status500_InternalServerError(
                models::ErrorResponse {
                    message: "Internal Server Error".to_string(),
                    error_code: "STORAGE_UPLOAD_FAILED".to_string(),
                    details: None,
                },
            ));
        }
    }

    Ok(
        apis::default::UploadImageResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: "success".to_string(),
                data: None,
            },
        ),
    )
}
