use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum GatewayError {
    BadGateway(String),
    GatewayTimeout(String),
    #[allow(dead_code)]
    PayloadTooLarge(String),
    InternalServerError(String),
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatewayError::BadGateway(ref msg) => write!(f, "Bad Gateway: {}", msg),
            GatewayError::GatewayTimeout(ref msg) => write!(f, "Gateway Timeout: {}", msg),
            GatewayError::PayloadTooLarge(ref msg) => write!(f, "Payload Too Large: {}", msg),
            GatewayError::InternalServerError(ref msg) => {
                write!(f, "Internal Server Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for GatewayError {}

#[derive(Serialize)]
struct ErrorPayload {
    pub error: String,
    pub message: String,
}

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        let (status, error_code, details) = match self {
            GatewayError::BadGateway(msg) => (StatusCode::BAD_GATEWAY, "BAD_GATEWAY", msg),
            GatewayError::GatewayTimeout(msg) => {
                (StatusCode::GATEWAY_TIMEOUT, "GATEWAY_TIMEOUT", msg)
            }
            GatewayError::PayloadTooLarge(msg) => {
                (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE", msg)
            }
            GatewayError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                msg,
            ),
        };

        let response_body = Json(ErrorPayload {
            error: error_code.to_string(),
            message: details,
        });

        (status, response_body).into_response()
    }
}
