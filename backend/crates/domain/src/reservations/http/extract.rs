use axum::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::request::Parts;
use axum::Json;
use kernel::envelope::Envelope;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::{Error, KernelState, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Body<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Body<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request(request: Request, state: &S) -> Result<Self> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

pub struct Params<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Params<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| Self(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

pub type Reply<T> = Result<Json<Envelope<T>>>;

pub fn reply<T: Serialize>(value: T, request_id: String) -> Json<Envelope<T>> {
    Json(Envelope::ok(value, request_id))
}

pub trait AppState: KernelState + Clone + Send + Sync + 'static {}

impl<T> AppState for T where T: KernelState + Clone + Send + Sync + 'static {}
