use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use generated::models;
use http::Method;
use log::{info, warn};

use crate::handler::messages::{error_code, error_message, success_message};
use crate::service::{ServiceError, UploadMergedImageService};

/// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
pub async fn handle(
    _method: &Method,
    _host: &Host,
    _cookies: &CookieJar,
    mut body: Multipart,
    service: &dyn UploadMergedImageService,
) -> Result<apis::default::UploadMergedImageResponse, ()> {
    info!("upload_merged_image() called");

    let mut signed_url: Option<String> = None;
    let mut files: Vec<Vec<u8>> = Vec::new();

    while let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        info!("field name: {}", name);
        if let Ok(data) = field.bytes().await {
            match name.as_str() {
                "signedUrl" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        signed_url = Some(s);
                    }
                }
                "files" => {
                    info!("file received: {} bytes", data.len());
                    files.push(data.to_vec());
                }
                _ => {
                    warn!("Unknown field: {}", name);
                }
            }
        } else {
            warn!("Failed to parse body to bytes");
        }
    }

    if signed_url.is_none() {
        return Ok(
            apis::default::UploadMergedImageResponse::Status400_BadRequest(models::ErrorResponse {
                message: error_message::BAD_REQUEST.to_string(),
                error_code: error_code::INVALID_INPUT.to_string(),
                details: None,
            }),
        );
    }

    if files.is_empty() {
        return Ok(
            apis::default::UploadMergedImageResponse::Status400_BadRequest(models::ErrorResponse {
                message: error_message::BAD_REQUEST.to_string(),
                error_code: error_code::INVALID_INPUT.to_string(),
                details: None,
            }),
        );
    }

    let signed_url = signed_url.unwrap();

    // NOTE: 実処理
    match service.execute(&signed_url, &files).await {
        Ok(_) => {}
        Err(ServiceError::Validation(msg)) => {
            info!("Validation error: {}", msg);
            return Ok(
                apis::default::UploadMergedImageResponse::Status400_BadRequest(
                    models::ErrorResponse {
                        message: error_message::BAD_REQUEST.to_string(),
                        error_code: error_code::INVALID_INPUT.to_string(),
                        details: None,
                    },
                ),
            );
        }
        Err(ServiceError::Infrastructure(e)) => {
            info!("Infrastructure error: {}", e);
            return Ok(
                apis::default::UploadMergedImageResponse::Status500_InternalServerError(
                    models::ErrorResponse {
                        message: error_message::INTERNAL_SERVER_ERROR.to_string(),
                        error_code: error_code::INFRASTRUCTURE_FAILED.to_string(),
                        details: None,
                    },
                ),
            );
        }
    }

    Ok(
        apis::default::UploadMergedImageResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: success_message::SUCCESS.to_string(),
                data: None,
            },
        ),
    )
}
