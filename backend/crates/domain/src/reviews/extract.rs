use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Query};
use kernel::error::{codes::MALFORMED_REQUEST, Error, Result};

pub fn body<T>(result: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    result.map(|Json(value)| value).map_err(|_| Error::bad_request(MALFORMED_REQUEST))
}

pub fn query<T>(result: std::result::Result<Query<T>, QueryRejection>) -> Result<T> {
    result.map(|Query(value)| value).map_err(|_| Error::bad_request(MALFORMED_REQUEST))
}
