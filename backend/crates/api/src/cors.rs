use anyhow::{Context, Result};
use axum::http::{header, HeaderValue, Method, Uri};
use tower_http::cors::CorsLayer;

pub fn layer(origins: &[String]) -> Result<CorsLayer> {
    let origins = origins.iter().map(|origin| {
        let uri: Uri = origin.parse().context("invalid CORS origin")?;
        let scheme = uri.scheme_str().filter(|scheme| matches!(*scheme, "http" | "https"))
            .context("invalid CORS origin scheme")?;
        let authority = uri.authority().context("invalid CORS origin authority")?;
        if authority.as_str().contains('@') || format!("{scheme}://{authority}") != origin.as_str() {
            anyhow::bail!("invalid CORS origin");
        }
        origin.parse::<HeaderValue>().context("invalid CORS origin header")
    }).collect::<Result<Vec<_>>>()?;
    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::HeaderName::from_static("x-city"), header::HeaderName::from_static("x-request-id")])
        .expose_headers([header::HeaderName::from_static("x-request-id")]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_explicit_origins_and_rejects_wildcards_or_paths() {
        assert!(layer(&["http://localhost:5273".into(), "https://example.com".into()]).is_ok());
        assert!(layer(&["*".into()]).is_err());
        assert!(layer(&["https://example.com/path".into()]).is_err());
        assert!(layer(&[]).is_ok());
    }
}
