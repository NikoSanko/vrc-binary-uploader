use thiserror::Error;

/// 画像モデルのエラー
#[derive(Debug, Error)]
pub enum ImageError {
    /// データが空
    #[error("image data is empty")]
    EmptyData,

    /// 画像のデコードエラー
    #[error("failed to decode image: {0}")]
    DecodeError(String),

    /// ピクセル数が4の倍数でない
    #[error("image dimensions must be multiples of 4 (width: {width}, height: {height})")]
    InvalidDimensions { width: u32, height: u32 },
}
