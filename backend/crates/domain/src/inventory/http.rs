use axum::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::request::Parts;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::Error;
use serde::de::DeserializeOwned;

pub struct Params<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Params<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| Params(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

pub struct JsonBody<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request(request: Request, state: &S) -> Result<Self, Error> {
        axum::Json::<T>::from_request(request, state)
            .await
            .map(|axum::Json(value)| JsonBody(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}
