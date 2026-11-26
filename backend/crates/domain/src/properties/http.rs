use axum::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::request::Parts;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::error::Result;
use kernel::paging::PageRequest;
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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> std::result::Result<Self, Error> {
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

    async fn from_request(request: Request, state: &S) -> std::result::Result<Self, Error> {
        axum::Json::<T>::from_request(request, state)
            .await
            .map(|axum::Json(value)| JsonBody(value))
            .map_err(|_| Error::bad_request(MALFORMED_REQUEST))
    }
}

pub fn paging(page: Option<u32>, page_size: Option<u32>) -> Result<PageRequest> {
    let default = PageRequest::default();
    PageRequest {
        page: page.unwrap_or(default.page),
        page_size: page_size.unwrap_or(default.page_size),
    }
    .validated()
}

pub fn like_pattern(value: Option<&str>) -> Option<String> {
    value.map(str::trim).filter(|text| !text.is_empty()).map(|text| {
        format!("%{}%", text.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn like_pattern_escapes_wildcards_and_skips_blanks() {
        assert_eq!(like_pattern(Some(" a_b% ")), Some("%a\\_b\\%%".to_string()));
        assert_eq!(like_pattern(Some("  ")), None);
        assert_eq!(like_pattern(None), None);
    }

    #[test]
    fn paging_uses_defaults_and_validates() {
        assert_eq!(paging(None, None).unwrap().page_size, 20);
        assert!(paging(Some(0), None).is_err());
        assert!(paging(None, Some(101)).is_err());
    }
}
