use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::INVENTORY_INTEGRATIONS;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use utoipa::ToSchema;

use super::view::feed_text;
use crate::inventory::messages::FEED_NOT_FOUND;
use crate::inventory::scope::host_property;

const TOKEN_PREFIX: &str = "ftk-";
const SECRET_LENGTH: usize = 64;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFeedTokenInput {
    pub property_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedTokenCreated {
    pub token_id: String,
    pub feed_path: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedTokenRevoked {
    pub token_id: String,
    pub revoked: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedTokenView {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub fn secret_hash(secret: &str) -> String {
    hex::encode(Sha256::digest(secret.as_bytes()))
}

pub fn split_token(token: &str) -> Result<(String, String)> {
    let (id, secret) = token.split_once('.').ok_or_else(|| Error::not_found(FEED_NOT_FOUND))?;
    if !id.starts_with(TOKEN_PREFIX)
        || secret.len() != SECRET_LENGTH
        || !secret.chars().all(|c| c.is_ascii_hexdigit())
    {
        return Err(Error::not_found(FEED_NOT_FOUND));
    }
    Ok((id.to_string(), secret.to_string()))
}

fn new_secret() -> String {
    format!("{}{}", &new_id("s")[2..], &new_id("s")[2..])
}

pub async fn create(pool: &PgPool, actor: &Actor, input: CreateFeedTokenInput) -> Result<FeedTokenCreated> {
    actor.require(INVENTORY_INTEGRATIONS)?;
    let property = host_property(pool, actor, &input.property_id, INVENTORY_INTEGRATIONS).await?;
    let token_id = format!("{TOKEN_PREFIX}{}", &new_id("t")[2..]);
    let secret = new_secret();
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO stay_calendar_feed_tokens (id, city_id, "propertyId", "secretHash", "createdByUserId")
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(&token_id)
    .bind(property.city.code())
    .bind(&property.id)
    .bind(secret_hash(&secret))
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "calendar.feed_token_create",
            resource_type: "property",
            resource_id: &property.id,
            reason: None,
            change: json!({ "tokenId": token_id }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(FeedTokenCreated {
        feed_path: format!("/api/public/calendar/feeds/{}/{}.{}.ics", property.city.code(), token_id, secret),
        token_id,
    })
}

pub async fn revoke(pool: &PgPool, actor: &Actor, token_id: &str) -> Result<FeedTokenRevoked> {
    actor.require(INVENTORY_INTEGRATIONS)?;
    let property_id: String = sqlx::query_scalar(
        r#"SELECT "propertyId" FROM stay_calendar_feed_tokens WHERE id = $1 AND "revokedAt" IS NULL"#,
    )
    .bind(token_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(FEED_NOT_FOUND))?;
    host_property(pool, actor, &property_id, INVENTORY_INTEGRATIONS).await?;
    let mut tx = pool.begin().await?;
    sqlx::query(r#"UPDATE stay_calendar_feed_tokens SET "revokedAt" = now() WHERE id = $1"#)
        .bind(token_id)
        .execute(&mut *tx)
        .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "calendar.feed_token_revoke",
            resource_type: "property",
            resource_id: &property_id,
            reason: None,
            change: json!({ "tokenId": token_id }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(FeedTokenRevoked { token_id: token_id.to_string(), revoked: true })
}

pub async fn list(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    page: PageRequest,
) -> Result<Page<FeedTokenView>> {
    actor.require(INVENTORY_INTEGRATIONS)?;
    let property = host_property(pool, actor, property_id, INVENTORY_INTEGRATIONS).await?;
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_calendar_feed_tokens WHERE "propertyId" = $1 AND "revokedAt" IS NULL"#,
    )
    .bind(&property.id)
    .fetch_one(pool)
    .await?;
    let items = sqlx::query_as::<_, FeedTokenView>(
        r#"SELECT id, "createdAt" AS created_at FROM stay_calendar_feed_tokens
           WHERE "propertyId" = $1 AND "revokedAt" IS NULL
           ORDER BY "createdAt" DESC, id LIMIT $2 OFFSET $3"#,
    )
    .bind(&property.id)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    Ok(Page::new(items, total, page))
}

pub async fn feed_by_token(pool: &PgPool, city: City, token: &str) -> Result<String> {
    let (token_id, secret) = split_token(token)?;
    let row: (String, String) = sqlx::query_as(
        r#"SELECT "propertyId", "secretHash" FROM stay_calendar_feed_tokens
           WHERE id = $1 AND city_id = $2 AND "revokedAt" IS NULL"#,
    )
    .bind(&token_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(FEED_NOT_FOUND))?;
    if row.1 != secret_hash(&secret) {
        return Err(Error::not_found(FEED_NOT_FOUND));
    }
    feed_text(pool, &row.0).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_a_well_formed_token() {
        let secret = "a".repeat(64);
        let (id, parsed) = split_token(&format!("ftk-abc.{secret}")).unwrap();
        assert_eq!(id, "ftk-abc");
        assert_eq!(parsed, secret);
    }

    #[test]
    fn rejects_malformed_tokens() {
        assert!(split_token("ftk-abc").is_err());
        assert!(split_token(&format!("other-abc.{}", "a".repeat(64))).is_err());
        assert!(split_token(&format!("ftk-abc.{}", "z".repeat(64))).is_err());
        assert!(split_token(&format!("ftk-abc.{}", "a".repeat(10))).is_err());
    }

    #[test]
    fn hashes_secrets_and_never_stores_them_plainly() {
        let hash = secret_hash("secret-value");
        assert_eq!(hash.len(), 64);
        assert_ne!(hash, "secret-value");
        assert_eq!(hash, secret_hash("secret-value"));
    }

    #[test]
    fn generated_secrets_are_valid_hex_of_the_expected_length() {
        let secret = new_secret();
        assert_eq!(secret.len(), SECRET_LENGTH);
        assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(split_token(&format!("ftk-x.{secret}")).is_ok());
    }
}
