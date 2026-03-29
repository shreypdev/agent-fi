use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: &'static str,
    pub message: String,
}

pub struct ApiError {
    pub status: StatusCode,
    pub body: ErrorBody,
    pub retry_after_secs: Option<u32>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut res = (self.status, Json(self.body)).into_response();
        if let Some(secs) = self.retry_after_secs {
            res.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                secs.to_string().parse().unwrap(),
            );
        }
        res
    }
}
