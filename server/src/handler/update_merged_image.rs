use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use generated::models;
use http::Method;
use log::info;

/// 複数画像を束ねたファイルの指定枚目だけ更新する
pub async fn handle(
    _method: &Method,
    _host: &Host,
    _cookies: &CookieJar,
    mut body: Multipart,
) -> Result<apis::default::UpdateMergedImageResponse, ()> {
    info!("update_merged_image() called");

    let mut presigned_url: Option<String> = None;
    let mut index: Option<i32> = None;
    let mut metadata: Option<String> = None;
    let mut _file_data: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if let Ok(data) = field.bytes().await {
            match name.as_str() {
                "presignedUrl" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        presigned_url = Some(s);
                    }
                }
                "index" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        if let Ok(i) = s.parse::<i32>() {
                            index = Some(i);
                        }
                    }
                }
                "metadata" => {
                    if let Ok(s) = String::from_utf8(data.to_vec()) {
                        metadata = Some(s);
                    }
                }
                "file" => {
                    _file_data = Some(data.to_vec());
                }
                _ => {
                    log::warn!("Unknown field: {}", name);
                }
            }
        }
    }

    if presigned_url.is_none() || index.is_none() || metadata.is_none() {
        return Ok(
            apis::default::UpdateMergedImageResponse::Status400_BadRequest(models::ErrorResponse {
                message: "Bad Request".to_string(),
                error_code: "INVALID_INPUT".to_string(),
                details: None,
            }),
        );
    }

    let _presigned_url = presigned_url.unwrap();
    let _index = index.unwrap();
    let _file_data = _file_data.unwrap_or_default();

    // TODO

    Ok(
        apis::default::UpdateMergedImageResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: "success".to_string(),
                data: None,
            },
        ),
    )
}
