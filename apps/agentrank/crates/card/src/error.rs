//! Parse errors for Agent Card validation.

/// Failure while parsing or validating an Agent Card.
#[derive(Debug, Clone, thiserror::Error)]
#[error("{kind}")]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

/// Specific validation failure (ordered for stable tests).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    NotJsonObject,
    InvalidJson(String),
    MissingField(&'static str),
    EmptyField(&'static str),
    InvalidEndpointUrl,
    InvalidEndpointScheme,
    InvalidSourceUrl,
    InvalidFinalUrl,
    InvalidProviderUrl,
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::NotJsonObject => write!(f, "root JSON value must be an object"),
            ParseErrorKind::InvalidJson(s) => write!(f, "invalid JSON structure: {s}"),
            ParseErrorKind::MissingField(name) => write!(f, "missing required field `{name}`"),
            ParseErrorKind::EmptyField(name) => {
                write!(f, "field `{name}` is empty or whitespace only")
            }
            ParseErrorKind::InvalidEndpointUrl => write!(f, "`url` is not a valid HTTP(S) URL"),
            ParseErrorKind::InvalidEndpointScheme => write!(f, "`url` must use http or https"),
            ParseErrorKind::InvalidSourceUrl => write!(f, "source_url is not a valid URL"),
            ParseErrorKind::InvalidFinalUrl => write!(f, "final_fetch_url is not a valid URL"),
            ParseErrorKind::InvalidProviderUrl => write!(f, "provider.url is not a valid URL"),
        }
    }
}
