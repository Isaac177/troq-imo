use axum::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::request::Parts;
use axum::Json;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::error::Error;
use serde::de::DeserializeOwned;

pub struct ApiQuery<T>(pub T);

pub struct ApiJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ApiQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        Query::<T>::try_from_uri(&parts.uri)
            .map(|Query(value)| Self(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(request: Request, state: &S) -> Result<Self, Error> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

pub trait Shared: kernel::kernel::KernelState + Clone + Send + Sync + 'static {}

impl<T> Shared for T where T: kernel::kernel::KernelState + Clone + Send + Sync + 'static {}
