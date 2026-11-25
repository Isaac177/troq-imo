use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::city::City;
use crate::error::codes::{AUTH_REQUIRED, SESSION_REVOKED};
use crate::error::{Error, Result};
use crate::kernel::Kernel;

pub const REQUEST_ID_HEADER: &str = "x-request-id";
pub const CITY_HEADER: &str = "x-city";

pub fn request_id(parts: &Parts) -> String {
    parts
        .headers
        .get(REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

pub fn requested_city(parts: &Parts) -> Option<String> {
    let header = parts
        .headers
        .get(CITY_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let query = || {
        parts.uri.query().and_then(|query| {
            query
                .split('&')
                .find_map(|pair| pair.strip_prefix("city=").map(str::to_string))
        })
    };
    header
        .or_else(query)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn resolve_city(parts: &Parts, kernel: &Kernel) -> Result<(City, bool)> {
    match requested_city(parts) {
        Some(code) => {
            let city = City::from_code(&code)?;
            if kernel.config.serves(city) {
                Ok((city, true))
            } else {
                Err(Error::bad_request(crate::error::codes::CITY_UNSUPPORTED)
                    .with_params(serde_json::json!({ "city": city })))
            }
        }
        None => Ok((kernel.config.default_city, false)),
    }
}

pub fn bearer_token(parts: &Parts) -> Result<&str> {
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .ok_or_else(|| Error::unauthorized(AUTH_REQUIRED))
}

pub fn session_revoked() -> Error {
    Error::unauthorized(SESSION_REVOKED)
}

pub fn is_write_method(parts: &Parts) -> bool {
    use axum::http::Method;
    matches!(parts.method, Method::POST | Method::PUT | Method::PATCH | Method::DELETE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    fn parts(uri: &str, header: Option<(&str, &str)>) -> Parts {
        let mut builder = Request::builder().uri(uri);
        if let Some((name, value)) = header {
            builder = builder.header(name, value);
        }
        builder.body(()).unwrap().into_parts().0
    }

    #[test]
    fn header_city_wins_over_query_city() {
        let parts = parts("/x?city=KIN", Some(("x-city", " goma ")));
        assert_eq!(requested_city(&parts).as_deref(), Some("goma"));
    }

    #[test]
    fn query_city_is_used_without_header() {
        assert_eq!(requested_city(&parts("/x?a=1&city=KIN", None)).as_deref(), Some("KIN"));
        assert_eq!(requested_city(&parts("/x", None)), None);
    }

    #[test]
    fn bearer_token_is_required() {
        assert!(bearer_token(&parts("/x", None)).is_err());
        let with = parts("/x", Some(("authorization", "Bearer abc")));
        assert_eq!(bearer_token(&with).unwrap(), "abc");
    }

    #[test]
    fn request_id_is_reused_or_generated() {
        assert_eq!(request_id(&parts("/x", Some(("x-request-id", "r-1")))), "r-1");
        assert!(!request_id(&parts("/x", None)).is_empty());
    }
}
