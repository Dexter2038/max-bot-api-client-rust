#![allow(clippy::enum_variant_names)]
use thiserror::Error;

/// Ошибки, которые могут возникнуть при работе с Max Messenger Bot API клиентом.
#[derive(Error, Debug)]
pub enum ClientError {
    /// Ошибка HTTP-транспорта (сетевая ошибка, таймаут, DNS и т.п.).
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Ошибка ввода-вывода (например, при чтении тела ответа).
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка парсинга JSON (несовпадение структуры, некорректный формат).
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Ошибка с неожиданным HTTP статусом, когда код ответа не успешный (не 2xx).
    #[error("Unexpected HTTP status: {0}")]
    StatusError(u16),
}
