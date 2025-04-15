use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("Failed to parse string in to JSON. Serde says: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Failed to parse. Unknown message type")]
    UnknownMessageType,
}
