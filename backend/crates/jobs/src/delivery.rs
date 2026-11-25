use std::collections::HashSet;

use async_trait::async_trait;
use kernel::{City, Error, Result};
use serde_json::Value;
use sqlx::{Acquire, PgPool, Postgres, Transaction};

use crate::backoff::retry_delay;
use crate::event::Event;

#[async_trait]
pub trait TransactionalSubscriber: Send + Sync {
    fn name(&self) -> &'static str;
    fn accepts(&self, event_type: &str) -> bool;
    async fn handle(&self, tx: &mut Transaction<'_, Postgres>, event: &Event) -> Result<()>;
}

async fn apply_subscribers(
    tx: &mut Transaction<'_, Postgres>,
    event: &Event,
    subscribers: &[&dyn TransactionalSubscriber],
) -> Result<()> {
    let interested: Vec<_> = subscribers.iter().filter(|subscriber| subscriber.accepts(&event.event_type)).collect();
    if interested.is_empty() {
        return Err(Error::internal("no subscriber registered for outbox event"));
    }
    for subscriber in interested {
        let inserted = sqlx::query(
            "INSERT INTO kernel_inbox (consumer_name, event_id, event_type) VALUES ($1, $2, $3) ON CONFLICT (consumer_name, event_id) DO NOTHING",
        )
        .bind(subscriber.name())
        .bind(&event.id)
        .bind(&event.event_type)
        .execute(&mut **tx)
        .await?;
        if inserted.rows_affected() == 1 {
            subscriber.handle(tx, event).await?;
        }
    }
    Ok(())
}

pub async fn deliver_one(pool: &PgPool, subscribers: &[&dyn TransactionalSubscriber]) -> Result<bool> {
    let mut names = HashSet::new();
    for subscriber in subscribers {
        if subscriber.name().is_empty() || !names.insert(subscriber.name()) {
            return Err(Error::internal("subscriber names must be nonempty and unique"));
        }
    }
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, (String, String, String, String, String, i32, Value, i32)>(
        "SELECT id, city_id, aggregate_type, aggregate_id, event_type, event_version, payload, attempts FROM kernel_outbox WHERE state IN ('PENDING', 'FAILED') AND next_attempt_at <= now() ORDER BY next_attempt_at, created_at, id LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_optional(&mut *tx)
    .await?;
    let Some((id, city, aggregate_type, aggregate_id, event_type, version, payload, attempts)) = row else {
        tx.commit().await?;
        return Ok(false);
    };
    let parsed = City::from_code(&city).and_then(|city| {
        let data = payload.get("data").cloned().ok_or_else(|| Error::internal("outbox payload missing data"))?;
        Ok(Event { id: id.clone(), city, aggregate_type, aggregate_id, event_type, version, data })
    });
    let outcome = match parsed {
        Ok(event) => {
            let mut savepoint = tx.begin().await?;
            match apply_subscribers(&mut savepoint, &event, subscribers).await {
                Ok(()) => {
                    savepoint.commit().await?;
                    Ok(())
                }
                Err(error) => {
                    savepoint.rollback().await?;
                    Err(error)
                }
            }
        }
        Err(error) => Err(error),
    };
    match outcome {
        Ok(()) => {
            sqlx::query("UPDATE kernel_outbox SET state = 'PUBLISHED', published_at = now(), last_error = NULL WHERE id = $1")
                .bind(&id).execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(true)
        }
        Err(error) => {
            let next_attempts = attempts.saturating_add(1);
            let delay_seconds = retry_delay(next_attempts).as_secs() as i64;
            sqlx::query("UPDATE kernel_outbox SET state = 'FAILED', attempts = $2, last_error = $3, next_attempt_at = now() + $4::bigint * interval '1 second' WHERE id = $1")
                .bind(&id).bind(next_attempts).bind(error.code().as_str()).bind(delay_seconds)
                .execute(&mut *tx).await?;
            tx.commit().await?;
            Err(error)
        }
    }
}
