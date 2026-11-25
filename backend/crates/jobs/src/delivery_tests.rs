use async_trait::async_trait;
use kernel::{Error, Result};
use sqlx::{PgPool, Postgres, Transaction};

use crate::delivery::{deliver_one, TransactionalSubscriber};
use crate::event::Event;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

struct Writer {
    fail: bool,
}

#[async_trait]
impl TransactionalSubscriber for Writer {
    fn name(&self) -> &'static str {
        "test-writer"
    }

    fn accepts(&self, event_type: &str) -> bool {
        event_type == "test.created"
    }

    async fn handle(&self, tx: &mut Transaction<'_, Postgres>, event: &Event) -> Result<()> {
        sqlx::query("INSERT INTO delivery_test_effects (event_id) VALUES ($1)")
            .bind(&event.id)
            .execute(&mut **tx)
            .await?;
        if self.fail {
            return Err(Error::internal("test failure"));
        }
        Ok(())
    }
}

async fn prepare(pool: &PgPool) {
    sqlx::query("CREATE TABLE delivery_test_effects (event_id text PRIMARY KEY)")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO kernel_outbox (id, city_id, aggregate_type, aggregate_id, event_type, payload) VALUES ('seed-event', 'GOMA', 'test', 'seed-object', 'test.created', '{\"data\":{}}'::jsonb)")
        .execute(pool)
        .await
        .unwrap();
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    sqlx::query_scalar(&sql).fetch_one(pool).await.unwrap()
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn failure_rolls_back_effect_and_inbox(pool: PgPool) {
    prepare(&pool).await;
    assert!(deliver_one(&pool, &[&Writer { fail: true }]).await.is_err());
    assert_eq!(count(&pool, "delivery_test_effects").await, 0);
    assert_eq!(count(&pool, "kernel_inbox").await, 0);
    let state: String = sqlx::query_scalar("SELECT state FROM kernel_outbox WHERE id = 'seed-event'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(state, "FAILED");
    let attempts: i32 = sqlx::query_scalar("SELECT attempts FROM kernel_outbox WHERE id = 'seed-event'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(attempts, 1);
    assert!(!deliver_one(&pool, &[&Writer { fail: false }]).await.unwrap());
    sqlx::query("UPDATE kernel_outbox SET next_attempt_at = now() WHERE id = 'seed-event'")
        .execute(&pool).await.unwrap();
    assert!(deliver_one(&pool, &[&Writer { fail: false }]).await.unwrap());
    assert_eq!(count(&pool, "delivery_test_effects").await, 1);
    assert_eq!(count(&pool, "kernel_inbox").await, 1);
    assert!(!deliver_one(&pool, &[&Writer { fail: false }]).await.unwrap());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn concurrent_workers_deliver_once(pool: PgPool) {
    prepare(&pool).await;
    let subscriber = Writer { fail: false };
    let subscribers: [&dyn TransactionalSubscriber; 1] = [&subscriber];
    let (first, second) = tokio::join!(deliver_one(&pool, &subscribers), deliver_one(&pool, &subscribers));
    assert_ne!(first.unwrap(), second.unwrap());
    assert_eq!(count(&pool, "delivery_test_effects").await, 1);
    assert_eq!(count(&pool, "kernel_inbox").await, 1);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn unknown_event_is_scheduled_for_retry(pool: PgPool) {
    prepare(&pool).await;
    assert!(deliver_one(&pool, &[]).await.is_err());
    let state: String = sqlx::query_scalar("SELECT state FROM kernel_outbox WHERE id = 'seed-event'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(state, "FAILED");
    assert_eq!(count(&pool, "kernel_inbox").await, 0);
}
