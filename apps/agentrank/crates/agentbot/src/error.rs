use agentrank_card::ParseError;
use thiserror::Error;

/// Ingest pipeline failure.
#[derive(Debug, Error)]
pub enum IngestError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("response body too large: {0} bytes (max {1})")]
    BodyTooLarge(usize, usize),

    #[error("non-success HTTP status: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("agent card parse error: {0}")]
    CardParse(#[from] ParseError),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("invalid fetch URL: {0}")]
    BadUrl(#[from] url::ParseError),
}
