/// エラーメッセージ
pub mod error_message {
    /// バリデーションエラーのメッセージ
    pub const BAD_REQUEST: &str = "Bad Request";

    /// 内部サーバーエラーのメッセージ
    pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
}

/// エラーコード
pub mod error_code {
    /// 無効な入力エラーコード
    pub const INVALID_INPUT: &str = "INVALID_INPUT";

    /// インフラストラクチャーエラーコード
    pub const INFRASTRUCTURE_FAILED: &str = "INFRASTRUCTURE_FAILED";
}

/// 成功メッセージ
pub mod success_message {
    /// 成功時のメッセージ
    pub const SUCCESS: &str = "success";
}
