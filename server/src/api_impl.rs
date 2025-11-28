use async_trait::async_trait;
use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use futures::StreamExt;
use generated::apis;
use generated::models;
use http::Method;
use log::info;

/// サーバー実装
#[derive(Clone)]
pub struct ServerImpl;

impl ServerImpl {
    pub fn new() -> Self {
        ServerImpl
    }
}

impl AsRef<ServerImpl> for ServerImpl {
    fn as_ref(&self) -> &ServerImpl {
        self
    }
}

#[async_trait]
impl apis::default::Default<()> for ServerImpl {
    /// 疎通確認
    async fn ping(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<apis::default::PingResponse, ()> {
        info!("ping() called");
        Ok(apis::default::PingResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: "pong".to_string(),
                data: None,
            },
        ))
    }

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする
    async fn upload_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        mut body: Multipart,
    ) -> Result<apis::default::UploadImageResponse, ()> {
        info!("upload_image() called");

        let mut signed_url: Option<String> = None;
        let mut metadata: Option<String> = None;
        let mut _file_data: Option<Vec<u8>> = None;

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
                    "file" => {
                        _file_data = Some(data.to_vec());
                    }
                    _ => {
                        log::warn!("Unknown field: {}", name);
                    }
                }
            }
        }

        if signed_url.is_none() || metadata.is_none() {
            return Ok(apis::default::UploadImageResponse::Status400_BadRequest(
                models::ErrorResponse {
                    message: "Bad Request".to_string(),
                    error_code: "INVALID_INPUT".to_string(),
                    details: None,
                },
            ));
        }

        // TODO: 実装を追加
        // - 画像をDDS形式に変換
        // - ストレージにアップロード

        Ok(apis::default::UploadImageResponse::Status200_SuccessfulOperation(
            models::SuccessResponse {
                message: "success".to_string(),
                data: None,
            },
        ))
    }

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
    async fn upload_merged_image(
        &self,
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
                apis::default::UploadMergedImageResponse::Status400_BadRequest(
                    models::ErrorResponse {
                        message: "Bad Request".to_string(),
                        error_code: "INVALID_INPUT".to_string(),
                        details: None,
                    },
                ),
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

    /// 複数画像を束ねたファイルの指定枚目だけ更新する
    async fn update_merged_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        mut body: Multipart,
    ) -> Result<apis::default::UpdateMergedImageResponse, ()> {
        info!("update_merged_image() called");

        let mut signed_url: Option<String> = None;
        let mut index: Option<i32> = None;
        let mut metadata: Option<String> = None;
        let mut _file_data: Option<Vec<u8>> = None;

        while let Ok(Some(field)) = body.next_field().await {
            let name = field.name().unwrap_or("").to_string();
            if let Ok(data) = field.bytes().await {
                match name.as_str() {
                    "signedUrl" => {
                        if let Ok(s) = String::from_utf8(data.to_vec()) {
                            signed_url = Some(s);
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

        if signed_url.is_none() || index.is_none() || metadata.is_none() {
            return Ok(
                apis::default::UpdateMergedImageResponse::Status400_BadRequest(
                    models::ErrorResponse {
                        message: "Bad Request".to_string(),
                        error_code: "INVALID_INPUT".to_string(),
                        details: None,
                    },
                ),
            );
        }

        // TODO: 実装を追加
        // - 指定されたインデックスの画像を更新
        // - DDS形式に変換
        // - ストレージにアップロード

        Ok(
            apis::default::UpdateMergedImageResponse::Status200_SuccessfulOperation(
                models::SuccessResponse {
                    message: "success".to_string(),
                    data: None,
                },
            ),
        )
    }
}

impl apis::ErrorHandler<()> for ServerImpl {}

