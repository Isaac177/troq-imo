use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Query};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::{Error, PageRequest, Result};
use serde::Serialize;

pub type Reply<T> = Result<Json<Envelope<T>>>;

pub fn reply<T: Serialize>(data: T, actor: &Actor) -> Reply<T> {
    Ok(Json(Envelope::ok(data, actor.request_id.clone())))
}

pub fn query<T>(result: std::result::Result<Query<T>, QueryRejection>) -> Result<T> {
    result.map(|Query(value)| value).map_err(|_| Error::bad_request(MALFORMED_REQUEST))
}

pub fn body<T>(result: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    result.map(|Json(value)| value).map_err(|_| Error::bad_request(MALFORMED_REQUEST))
}

pub fn paging(page: Option<u32>, page_size: Option<u32>) -> PageRequest {
    let default = PageRequest::default();
    PageRequest { page: page.unwrap_or(default.page), page_size: page_size.unwrap_or(default.page_size) }
}
