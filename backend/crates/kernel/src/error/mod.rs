pub mod codes;

use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

pub use codes::ErrorCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Detail {
    pub code: ErrorCode,
    pub params: Option<Value>,
}

impl Detail {
    fn new(code: ErrorCode) -> Self {
        Self { code, params: None }
    }
}

#[derive(Debug)]
pub enum Error {
    BadRequest(Detail),
    Unauthorized(Detail),
    Forbidden(Detail),
    NotFound(Detail),
    Conflict(Detail),
    Internal(String),
}

impl Error {
    pub fn bad_request(code: ErrorCode) -> Self {
        Self::BadRequest(Detail::new(code))
    }

    pub fn unauthorized(code: ErrorCode) -> Self {
        Self::Unauthorized(Detail::new(code))
    }

    pub fn forbidden(code: ErrorCode) -> Self {
        Self::Forbidden(Detail::new(code))
    }

    pub fn not_found(code: ErrorCode) -> Self {
        Self::NotFound(Detail::new(code))
    }

    pub fn conflict(code: ErrorCode) -> Self {
        Self::Conflict(Detail::new(code))
    }

    pub fn internal(detail: impl Into<String>) -> Self {
        Self::Internal(detail.into())
    }

    pub fn with_params(mut self, params: Value) -> Self {
        match &mut self {
            Self::BadRequest(detail)
            | Self::Unauthorized(detail)
            | Self::Forbidden(detail)
            | Self::NotFound(detail)
            | Self::Conflict(detail) => detail.params = Some(params),
            Self::Internal(_) => {}
        }
        self
    }

    pub fn city_mismatch(route: &str, resource: &str) -> Self {
        Self::forbidden(codes::CITY_MISMATCH)
            .with_params(json!({ "route": route, "resource": resource }))
    }

    pub fn permission_missing(permission: &str) -> Self {
        Self::forbidden(codes::PERMISSION_MISSING).with_params(json!({ "permission": permission }))
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            Self::BadRequest(detail)
            | Self::Unauthorized(detail)
            | Self::Forbidden(detail)
            | Self::NotFound(detail)
            | Self::Conflict(detail) => detail.code,
            Self::Internal(_) => codes::INTERNAL_ERROR,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn params(&self) -> Option<&Value> {
        match self {
            Self::BadRequest(detail)
            | Self::Unauthorized(detail)
            | Self::Forbidden(detail)
            | Self::NotFound(detail)
            | Self::Conflict(detail) => detail.params.as_ref(),
            Self::Internal(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(detail) => write!(formatter, "internal error: {detail}"),
            other => formatter.write_str(other.code().as_str()),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Serialize)]
struct ErrorBody {
    success: bool,
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        if let Self::Internal(detail) = &self {
            tracing::error!(detail = %detail, "request failed");
        }
        let body = ErrorBody {
            success: false,
            error: ErrorDetail {
                code: self.code(),
                params: self.params().cloned(),
            },
        };
        (self.status(), Json(body)).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_variant_maps_to_one_status_and_code() {
        assert_eq!(Error::bad_request(codes::PAGE_INVALID).status(), StatusCode::BAD_REQUEST);
        assert_eq!(Error::unauthorized(codes::AUTH_REQUIRED).status(), StatusCode::UNAUTHORIZED);
        assert_eq!(Error::forbidden(codes::PERMISSION_MISSING).status(), StatusCode::FORBIDDEN);
        assert_eq!(Error::not_found(codes::NOT_FOUND).status(), StatusCode::NOT_FOUND);
        assert_eq!(Error::conflict(codes::NOT_FOUND).status(), StatusCode::CONFLICT);
        assert_eq!(Error::internal("x").status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(Error::internal("x").code().as_str(), "INTERNAL_ERROR");
    }

    #[test]
    fn city_mismatch_is_forbidden_with_both_cities() {
        let error = Error::city_mismatch("GOMA", "KIN");
        assert_eq!(error.status(), StatusCode::FORBIDDEN);
        assert_eq!(error.code().as_str(), "CITY_MISMATCH");
        assert_eq!(error.params().unwrap()["resource"], "KIN");
    }

    #[test]
    fn internal_detail_is_never_part_of_the_body() {
        assert!(Error::internal("secret").params().is_none());
    }
}
