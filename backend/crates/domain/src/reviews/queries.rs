use chrono::{DateTime, Utc};
use kernel::city::City;
use kernel::error::{Error, Result};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use super::codes::{REVIEW_ALREADY_EXISTS, REVIEW_RESERVATION_NOT_FOUND};
use super::sql::REVIEW_COLUMNS;
use super::types::ReviewView;

#[derive(Debug, FromRow)]
pub struct ReviewRow {
    pub id: String,
    pub reservation_id: String,
    pub property_id: String,
    pub operator_id: String,
    pub rating: i16,
    pub comment: Option<String>,
    pub status: String,
    pub operator_reply: Option<String>,
    pub operator_replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub unit_type_id: Option<String>,
    pub unit_type_name_en: Option<String>,
    pub unit_type_name_fr: Option<String>,
}

impl ReviewRow {
    pub fn into_view(self) -> ReviewView {
        ReviewView {
            id: self.id,
            property_id: self.property_id,
            rating: self.rating,
            comment: self.comment,
            status: self.status,
            operator_reply: self.operator_reply,
            operator_replied_at: self.operator_replied_at,
            created_at: self.created_at,
            unit_type_id: self.unit_type_id,
            unit_type_name_en: self.unit_type_name_en,
            unit_type_name_fr: self.unit_type_name_fr,
            unit_type_cover_url: None,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PartyRow {
    pub guest_user_id: String,
    pub operator_id: String,
    pub property_id: String,
    pub state: String,
}

pub async fn party_of(pool: &PgPool, city: City, reservation_id: &str) -> Result<PartyRow> {
    sqlx::query_as::<_, PartyRow>(
        r#"SELECT "guestUserId" AS guest_user_id, "operatorId" AS operator_id,
                  "propertyId" AS property_id, state
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(REVIEW_RESERVATION_NOT_FOUND))
}

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    review_id: &str,
    reservation_id: &str,
    party: &PartyRow,
    rating: i16,
    comment: Option<&str>,
) -> Result<ReviewRow> {
    let sql = format!(
        r#"INSERT INTO stay_reviews
           (id, city_id, "reservationId", "propertyId", "operatorId", "guestUserId", rating, comment)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING {REVIEW_COLUMNS}"#
    );
    sqlx::query_as::<_, ReviewRow>(&sql)
        .bind(review_id)
        .bind(city.code())
        .bind(reservation_id)
        .bind(&party.property_id)
        .bind(&party.operator_id)
        .bind(&party.guest_user_id)
        .bind(rating)
        .bind(comment)
        .fetch_one(&mut **tx)
        .await
        .map_err(map_insert_error)
}

fn map_insert_error(error: sqlx::Error) -> Error {
    if let sqlx::Error::Database(database) = &error {
        if database.is_unique_violation() {
            return Error::conflict(REVIEW_ALREADY_EXISTS);
        }
    }
    Error::from(error)
}

pub async fn find(pool: &PgPool, city: City, review_id: &str) -> Result<Option<ReviewRow>> {
    let sql = format!(
        "SELECT {REVIEW_COLUMNS} FROM stay_reviews WHERE stay_reviews.id = $1 AND stay_reviews.city_id = $2"
    );
    Ok(sqlx::query_as::<_, ReviewRow>(&sql)
        .bind(review_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?)
}

pub async fn property_host(pool: &PgPool, city: City, property_id: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar::<_, String>(
        r#"SELECT "operatorId" FROM stay_properties WHERE id = $1 AND city_id = $2"#,
    )
    .bind(property_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?)
}

pub async fn set_reply(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    review_id: &str,
    reply: &str,
) -> Result<Option<ReviewRow>> {
    let sql = format!(
        r#"UPDATE stay_reviews
           SET "operatorReply" = $3, "operatorRepliedAt" = now(), "updatedAt" = now()
           WHERE id = $1 AND city_id = $2
           RETURNING {REVIEW_COLUMNS}"#
    );
    Ok(sqlx::query_as::<_, ReviewRow>(&sql)
        .bind(review_id)
        .bind(city.code())
        .bind(reply)
        .fetch_optional(&mut **tx)
        .await?)
}

pub async fn lock_status(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    review_id: &str,
) -> Result<Option<String>> {
    Ok(sqlx::query_scalar::<_, String>(
        "SELECT status FROM stay_reviews WHERE id = $1 AND city_id = $2 FOR UPDATE",
    )
    .bind(review_id)
    .bind(city.code())
    .fetch_optional(&mut **tx)
    .await?)
}

pub async fn set_status(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    review_id: &str,
    status: &str,
    moderator_id: &str,
) -> Result<ReviewRow> {
    let sql = format!(
        r#"UPDATE stay_reviews
           SET status = $3, "moderatedBy" = $4, "moderatedAt" = now(), "updatedAt" = now()
           WHERE id = $1 AND city_id = $2
           RETURNING {REVIEW_COLUMNS}"#
    );
    Ok(sqlx::query_as::<_, ReviewRow>(&sql)
        .bind(review_id)
        .bind(city.code())
        .bind(status)
        .bind(moderator_id)
        .fetch_one(&mut **tx)
        .await?)
}
