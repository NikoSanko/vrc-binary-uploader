use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PingResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateMergedImageResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UploadImageResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UploadMergedImageResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::ErrorResponse)
}




/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 疎通確認.
    ///
    /// Ping - GET /api/v1/ping
    async fn ping(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<PingResponse, E>;

    /// 複数画像を束ねたファイルの指定枚目だけ更新する.
    ///
    /// UpdateMergedImage - PUT /api/v1/merged-images
    async fn update_merged_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    body: Multipart,
    ) -> Result<UpdateMergedImageResponse, E>;

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする.
    ///
    /// UploadImage - POST /api/v1/images
    async fn upload_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    body: Multipart,
    ) -> Result<UploadImageResponse, E>;

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする.
    ///
    /// UploadMergedImage - POST /api/v1/merged-images
    async fn upload_merged_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    body: Multipart,
    ) -> Result<UploadMergedImageResponse, E>;
}
