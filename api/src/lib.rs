#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, unused_attributes, non_camel_case_types)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::collections::BTreeSet;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};
use crate::server::Authorization;


type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/api/v1";
pub const API_VERSION: &str = "1.0.0";

mod auth;
pub use auth::{AuthenticationApi, Claims};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PingResponse {
    /// Successful operation
    SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateMergedImageResponse {
    /// Successful operation
    SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UploadImageResponse {
    /// Successful operation
    SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UploadMergedImageResponse {
    /// Successful operation
    SuccessfulOperation
    (models::SuccessResponse)
    ,
    /// Bad Request
    BadRequest
    (models::ErrorResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::ErrorResponse)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    /// 疎通確認
    async fn ping(
        &self,
        context: &C) -> Result<PingResponse, ApiError>;

    /// 複数画像を束ねたファイルの指定枚目だけ更新する
    async fn update_merged_image(
        &self,
        signed_url: String,
        index: i32,
        metadata: String,
        file: swagger::ByteArray,
        context: &C) -> Result<UpdateMergedImageResponse, ApiError>;

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする
    async fn upload_image(
        &self,
        signed_url: String,
        metadata: String,
        file: Option<swagger::ByteArray>,
        context: &C) -> Result<UploadImageResponse, ApiError>;

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
    async fn upload_merged_image(
        &self,
        signed_url: String,
        metadata: String,
        files: Option<&Vec<models::File>>,
        context: &C) -> Result<UploadMergedImageResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn context(&self) -> &C;

    /// 疎通確認
    async fn ping(
        &self,
        ) -> Result<PingResponse, ApiError>;

    /// 複数画像を束ねたファイルの指定枚目だけ更新する
    async fn update_merged_image(
        &self,
        signed_url: String,
        index: i32,
        metadata: String,
        file: swagger::ByteArray,
        ) -> Result<UpdateMergedImageResponse, ApiError>;

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする
    async fn upload_image(
        &self,
        signed_url: String,
        metadata: String,
        file: Option<swagger::ByteArray>,
        ) -> Result<UploadImageResponse, ApiError>;

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
    async fn upload_merged_image(
        &self,
        signed_url: String,
        metadata: String,
        files: Option<&Vec<models::File>>,
        ) -> Result<UploadMergedImageResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// 疎通確認
    async fn ping(
        &self,
        ) -> Result<PingResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().ping(&context).await
    }

    /// 複数画像を束ねたファイルの指定枚目だけ更新する
    async fn update_merged_image(
        &self,
        signed_url: String,
        index: i32,
        metadata: String,
        file: swagger::ByteArray,
        ) -> Result<UpdateMergedImageResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_merged_image(signed_url, index, metadata, file, &context).await
    }

    /// １枚の画像をDDS形式に変換し、ストレージにアップロードする
    async fn upload_image(
        &self,
        signed_url: String,
        metadata: String,
        file: Option<swagger::ByteArray>,
        ) -> Result<UploadImageResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().upload_image(signed_url, metadata, file, &context).await
    }

    /// 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする
    async fn upload_merged_image(
        &self,
        signed_url: String,
        metadata: String,
        files: Option<&Vec<models::File>>,
        ) -> Result<UploadMergedImageResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().upload_merged_image(signed_url, metadata, files, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
