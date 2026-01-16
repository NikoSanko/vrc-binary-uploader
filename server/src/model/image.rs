use crate::model::error::ImageError;
use image::GenericImageView;

/// 画像情報を表すモデル
#[derive(Debug, Clone)]
pub struct Image {
    /// 画像のバイトデータ
    pub data: Vec<u8>,
}

impl Image {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[test]
    fn データが空ならエラーを返す() {
        let result = Image::try_from(&[] as &[u8]);
        assert!(matches!(result, Err(ImageError::EmptyData)));
    }

    #[test]
    fn 無効な画像データならデコードエラーを返す() {
        let invalid_data = vec![0, 1, 2, 3, 4, 5];
        let result = Image::try_from(invalid_data.as_slice());
        assert!(matches!(result, Err(ImageError::DecodeError(_))));
    }

    #[tokio::test]
    async fn 四の倍数のサイズなら成功する() {
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = Image::try_from(jpeg_data.as_slice());
        assert!(result.is_ok());
        let image = result.unwrap();
        assert!(!image.is_empty());
        assert_eq!(image.as_bytes().len(), jpeg_data.len());
    }

    #[tokio::test]
    async fn 幅が四の倍数でないサイズならエラーを返す() {
        let jpeg_data = fs::read("resources/not_4_multiple_width.jpg")
            .await
            .unwrap();
        let result = Image::try_from(jpeg_data.as_slice());
        assert!(matches!(result, Err(ImageError::InvalidDimensions { .. })));
        if let Err(ImageError::InvalidDimensions { width, height: _ }) = result {
            assert!(width % 4 != 0);
        }
    }

    #[tokio::test]
    async fn 高さが四の倍数でないサイズならエラーを返す() {
        let jpeg_data = fs::read("resources/not_4_multiple_height.jpg")
            .await
            .unwrap();
        let result = Image::try_from(jpeg_data.as_slice());
        assert!(matches!(result, Err(ImageError::InvalidDimensions { .. })));
        if let Err(ImageError::InvalidDimensions { width: _, height }) = result {
            assert!(height % 4 != 0);
        }
    }

    #[tokio::test]
    async fn vec_u8からも変換できる() {
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = Image::try_from(jpeg_data);
        assert!(result.is_ok());

        let empty_data = vec![];
        let result = Image::try_from(empty_data);
        assert!(matches!(result, Err(ImageError::EmptyData)));
    }
}
