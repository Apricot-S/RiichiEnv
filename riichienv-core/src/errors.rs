#[derive(Debug, thiserror::Error)]
pub enum RiichiError {
    /// 牌文字列・手牌文字列のパースエラー
    #[error("Parse error on '{input}': {message}")]
    Parse { input: String, message: String },
    /// アクション構成・エンコードのバリデーションエラー
    #[error("Invalid action: {message}")]
    InvalidAction { message: String },
    /// ゲーム状態の不整合（リプレイ同期ずれ等）
    #[error("Invalid state: {message}")]
    InvalidState { message: String },
    /// シリアライズ/デシリアライズの失敗
    #[error("Serialization error: {message}")]
    Serialization { message: String },
}

pub type RiichiResult<T> = Result<T, RiichiError>;

#[cfg(feature = "python")]
impl From<RiichiError> for pyo3::PyErr {
    fn from(err: RiichiError) -> pyo3::PyErr {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}
