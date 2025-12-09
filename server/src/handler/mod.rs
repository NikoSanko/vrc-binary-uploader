use async_trait::async_trait;
use axum::extract::Multipart;
use axum_extra::extract::{CookieJar, Host};
use generated::apis;
use http::Method;
use std::sync::Arc;

use crate::service::UploadSingleImageService;

mod ping;
mod update_merged_image;
mod upload_image;
mod upload_merged_image;

/// サーバー実装
#[derive(Clone)]
pub struct ServerImpl {
    upload_image_service: Arc<dyn UploadSingleImageService>,
}

impl ServerImpl {
    pub fn new(upload_image_service: Arc<dyn UploadSingleImageService>) -> Self {
        Self {
            upload_image_service,
        }
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
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<apis::default::PingResponse, ()> {
        ping::handle(method, host, cookies).await
    }

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする
    async fn upload_image(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: Multipart,
    ) -> Result<apis::default::UploadImageResponse, ()> {
        upload_image::handle(
            method,
            host,
            cookies,
            body,
            self.upload_image_service.as_ref(),
        )
        .await
    }

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
    async fn upload_merged_image(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: Multipart,
    ) -> Result<apis::default::UploadMergedImageResponse, ()> {
        upload_merged_image::handle(method, host, cookies, body).await
    }

    /// 複数画像を束ねたファイルの指定枚目だけ更新する
    async fn update_merged_image(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: Multipart,
    ) -> Result<apis::default::UpdateMergedImageResponse, ()> {
        update_merged_image::handle(method, host, cookies, body).await
    }
}

impl apis::ErrorHandler<()> for ServerImpl {}
