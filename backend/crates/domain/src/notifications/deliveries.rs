use kernel::ids::new_id;
use kernel::{City, Result};
use serde_json::Value;
use sqlx::{PgConnection, Postgres, Transaction};

pub const MAX_ATTEMPTS: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Push,
    Email,
    Whatsapp,
    Document,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Push => "PUSH",
            Self::Email => "EMAIL",
            Self::Whatsapp => "WHATSAPP",
            Self::Document => "DOCUMENT",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "PUSH" => Some(Self::Push),
            "EMAIL" => Some(Self::Email),
            "WHATSAPP" => Some(Self::Whatsapp),
            "DOCUMENT" => Some(Self::Document),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Delivery {
    pub id: String,
    pub city: String,
    pub channel: String,
    pub user_id: Option<String>,
    pub notification_id: Option<String>,
    pub payload: Value,
    pub attempts: i32,
}

pub struct NewDelivery<'a> {
    pub city: City,
    pub dedupe_key: &'a str,
    pub channel: Channel,
    pub user_id: Option<&'a str>,
    pub notification_id: Option<&'a str>,
    pub payload: Value,
}

pub async fn enqueue(conn: &mut PgConnection, input: NewDelivery<'_>) -> Result<()> {
    sqlx::query(
        "INSERT INTO notification_deliveries (id, city_id, dedupe_key, channel, user_id, notification_id, payload) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (dedupe_key) DO NOTHING",
    )
    .bind(new_id("delivery"))
    .bind(input.city.code())
    .bind(input.dedupe_key)
    .bind(input.channel.as_str())
    .bind(input.user_id)
    .bind(input.notification_id)
    .bind(input.payload)
    .execute(conn)
    .await?;
    Ok(())
}

type Row = (String, String, String, Option<String>, Option<String>, Value, i32);

pub async fn claim_next(tx: &mut Transaction<'_, Postgres>, channels: &[&str]) -> Result<Option<Delivery>> {
    let channels: Vec<String> = channels.iter().map(|channel| channel.to_string()).collect();
    let row = sqlx::query_as::<_, Row>(
        "SELECT id, city_id, channel, user_id, notification_id, payload, attempts FROM notification_deliveries WHERE state = 'PENDING' AND next_attempt_at <= now() AND channel = ANY($1) ORDER BY next_attempt_at, created_at, id LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .bind(&channels)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(row.map(|(id, city, channel, user_id, notification_id, payload, attempts)| Delivery {
        id,
        city,
        channel,
        user_id,
        notification_id,
        payload,
        attempts,
    }))
}

pub async fn mark_done(tx: &mut Transaction<'_, Postgres>, id: &str) -> Result<()> {
    sqlx::query("UPDATE notification_deliveries SET state = 'DONE', done_at = now(), last_error = NULL WHERE id = $1")
        .bind(id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn mark_skipped(tx: &mut Transaction<'_, Postgres>, id: &str, reason: &str) -> Result<()> {
    sqlx::query("UPDATE notification_deliveries SET state = 'SKIPPED', done_at = now(), last_error = $2 WHERE id = $1")
        .bind(id)
        .bind(reason)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub fn retry_delay_seconds(attempts: i32) -> i64 {
    let exponent = attempts.clamp(1, 8) as u32;
    (30_i64 * 2_i64.pow(exponent - 1)).min(3600)
}

pub async fn mark_failed(tx: &mut Transaction<'_, Postgres>, delivery: &Delivery, error: &str) -> Result<()> {
    let attempts = delivery.attempts + 1;
    let exhausted = attempts >= MAX_ATTEMPTS;
    sqlx::query(
        "UPDATE notification_deliveries SET state = CASE WHEN $4 THEN 'FAILED' ELSE 'PENDING' END, attempts = $2, last_error = $3, next_attempt_at = now() + $5::bigint * interval '1 second' WHERE id = $1",
    )
    .bind(&delivery.id)
    .bind(attempts)
    .bind(error)
    .bind(exhausted)
    .bind(retry_delay_seconds(attempts))
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_names_round_trip() {
        for channel in [Channel::Push, Channel::Email, Channel::Whatsapp, Channel::Document] {
            assert_eq!(Channel::parse(channel.as_str()), Some(channel));
        }
        assert_eq!(Channel::parse("SMS"), None);
    }

    #[test]
    fn retry_delay_doubles_and_is_capped() {
        assert_eq!(retry_delay_seconds(1), 30);
        assert_eq!(retry_delay_seconds(2), 60);
        assert_eq!(retry_delay_seconds(8), 3600);
        assert_eq!(retry_delay_seconds(50), 3600);
    }
}
