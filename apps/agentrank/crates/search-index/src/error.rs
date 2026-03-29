//! Index errors.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("tantivy: {0}")]
    Tantivy(#[from] tantivy::TantivyError),

    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("invalid index directory: {0}")]
    BadPath(String),

    #[error("schema version mismatch: index has {found}, expected {expected}")]
    SchemaVersion { found: String, expected: String },

    #[error("query parse: {0}")]
    QueryParse(tantivy::query::QueryParserError),

    #[error("index already exists at {0}; remove it or use rebuild")]
    IndexExists(String),
}
