use crate::model::error::ImageError;
use image::GenericImageView;

/// 画像情報を表すモデル
#[derive(Debug, Clone)]
pub struct Image {
    /// 画像のバイトデータ
    pub data: Vec<u8>,
}

impl Image {
    /// 新しい画像インスタンスを作成（バリデーション付き）
    ///
    /// # エラー
    /// - データが空の場合: `ImageError::EmptyData`
    /// - 画像のデコードに失敗した場合: `ImageError::DecodeError`
    /// - 縦横のピクセル数が4の倍数でない場合: `ImageError::InvalidDimensions`
    pub fn try_new(data: Vec<u8>) -> Result<Self, ImageError> {
        Self::try_from(data.as_slice())
    }

    /// 画像データが空かどうかを確認
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 画像データへの参照を取得
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl std::convert::TryFrom<Vec<u8>> for Image {
    type Error = ImageError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(data.as_slice())
    }
}

impl std::convert::TryFrom<&[u8]> for Image {
    type Error = ImageError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // データが空かチェック
        if data.is_empty() {
            return Err(ImageError::EmptyData);
        }

        // デコードしてサイズを取得
        let img =
            image::load_from_memory(data).map_err(|e| ImageError::DecodeError(e.to_string()))?;

        let (width, height) = img.dimensions();

        // 縦横のピクセル数が4の倍数かチェック
        if width % 4 != 0 || height % 4 != 0 {
            return Err(ImageError::InvalidDimensions { width, height });
        }

        Ok(Self {
            data: data.to_vec(),
        })
    }
}
