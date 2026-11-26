use std::collections::HashSet;

use kernel::error::{Error, ErrorCode, Result};
use serde_json::json;
use sqlx::PgPool;

pub const AMENITY_UNKNOWN: ErrorCode = ErrorCode::new("AMENITY_UNKNOWN");

pub async fn ensure_known(pool: &PgPool, codes: &[String]) -> Result<()> {
    if codes.is_empty() {
        return Ok(());
    }
    let known: HashSet<String> = sqlx::query_scalar("SELECT code FROM property_amenities WHERE code = ANY($1)")
        .bind(codes)
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect();
    match codes.iter().find(|code| !known.contains(*code)) {
        Some(code) => Err(Error::bad_request(AMENITY_UNKNOWN).with_params(json!({ "code": code }))),
        None => Ok(()),
    }
}
