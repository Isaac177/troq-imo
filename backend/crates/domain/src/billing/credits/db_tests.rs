use kernel::City;
use sqlx::{PgPool, Postgres, Transaction};

use super::charge;
use super::ledger::{self, EntryType, NewEntry};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

const HOST: &str = "seed-host-bill-test";

async fn prepare(pool: &PgPool) {
    sqlx::query(
        "INSERT INTO hosts (id, city_id, display_name, kind, status) VALUES ($1, 'GOMA', 'Credit Test Host', 'BUSINESS', 'ACTIVE')",
    )
    .bind(HOST)
    .execute(pool)
    .await
    .unwrap();
}

async fn balance(tx: &mut Transaction<'_, Postgres>) -> i64 {
    sqlx::query_scalar(
        r#"SELECT COALESCE((SELECT (balance * 100)::bigint FROM stay_credit_accounts WHERE "operatorId" = $1), 0)"#,
    )
    .bind(HOST)
    .fetch_one(&mut **tx)
    .await
    .unwrap()
}

async fn fund(tx: &mut Transaction<'_, Postgres>, minor: i64) {
    ledger::apply(
        tx,
        NewEntry {
            city: City::GOMA,
            host_id: HOST,
            entry_type: EntryType::Topup,
            amount_minor: minor,
            reservation_id: None,
            reason_code: Some("CASH_RECEIVED"),
            payment_ref: Some("test"),
            note: None,
            created_by: None,
        },
    )
    .await
    .unwrap();
}

async fn reservation(tx: &mut Transaction<'_, Postgres>, suffix: &str, commission: &str, penalty: &str) -> String {
    let id = format!("seed-res-bill-test-{suffix}");
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode",
                                          "totalAmount", "contractSnapshot", "commissionRateBps", "commissionAmount", "cancellationChargeAmount")
           VALUES ($1, $2, 'GOMA', 'seed-prop-bill-test', $3, 'seed-guest-bill-test', 'CONFIRMED', 'INSTANT',
                   300000.00, '{}', 1000, $4::numeric, $5::numeric)"#,
    )
    .bind(&id)
    .bind(format!("CRD-{suffix}"))
    .bind(HOST)
    .bind(commission)
    .bind(penalty)
    .execute(&mut **tx)
    .await
    .unwrap();
    id
}

async fn entries(tx: &mut Transaction<'_, Postgres>, id: &str, kind: &str) -> i64 {
    sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_credit_entries WHERE "reservationId" = $1 AND "entryType" = $2"#)
        .bind(id)
        .bind(kind)
        .fetch_one(&mut **tx)
        .await
        .unwrap()
}

async fn charged(tx: &mut Transaction<'_, Postgres>, id: &str) -> i64 {
    sqlx::query_scalar(r#"SELECT ("commissionChargedAmount" * 100)::bigint FROM stay_reservations WHERE id = $1"#)
        .bind(id)
        .fetch_one(&mut **tx)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn commission_is_taken_once_and_refunded_once(pool: PgPool) {
    prepare(&pool).await;
    let mut tx = pool.begin().await.unwrap();
    fund(&mut tx, 10_000_000).await;
    let start = balance(&mut tx).await;
    let id = reservation(&mut tx, "a", "30000.00", "0").await;
    charge::take_commission(&mut tx, &id).await.unwrap();
    charge::take_commission(&mut tx, &id).await.unwrap();
    assert_eq!(balance(&mut tx).await, start - 3_000_000);
    assert_eq!(entries(&mut tx, &id, "COMMISSION").await, 1);
    assert_eq!(charged(&mut tx, &id).await, 3_000_000);
    charge::release_commission(&mut tx, &id, 0).await.unwrap();
    charge::release_commission(&mut tx, &id, 0).await.unwrap();
    assert_eq!(balance(&mut tx).await, start);
    assert_eq!(entries(&mut tx, &id, "REFUND").await, 1);
    assert_eq!(charged(&mut tx, &id).await, 0);
    tx.rollback().await.unwrap();
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn a_penalty_keeps_commission_only_on_the_penalty_charge(pool: PgPool) {
    prepare(&pool).await;
    let mut tx = pool.begin().await.unwrap();
    fund(&mut tx, 10_000_000).await;
    let start = balance(&mut tx).await;
    let id = reservation(&mut tx, "b", "30000.00", "100000.00").await;
    charge::take_commission(&mut tx, &id).await.unwrap();
    charge::release_for_penalty(&mut tx, &id).await.unwrap();
    charge::release_for_penalty(&mut tx, &id).await.unwrap();
    assert_eq!(balance(&mut tx).await, start - 1_000_000);
    assert_eq!(charged(&mut tx, &id).await, 1_000_000);
    tx.rollback().await.unwrap();
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn an_amendment_charges_or_refunds_the_commission_difference(pool: PgPool) {
    prepare(&pool).await;
    let mut tx = pool.begin().await.unwrap();
    fund(&mut tx, 10_000_000).await;
    let start = balance(&mut tx).await;
    let id = reservation(&mut tx, "c", "30000.00", "0").await;
    charge::take_commission(&mut tx, &id).await.unwrap();
    sqlx::query(r#"UPDATE stay_reservations SET "commissionAmount" = 40000.00, "operatorNetAmount" = 260000.00 WHERE id = $1"#)
        .bind(&id)
        .execute(&mut *tx)
        .await
        .unwrap();
    charge::sync_amended_commission(&mut tx, &id).await.unwrap();
    assert_eq!(balance(&mut tx).await, start - 4_000_000);
    sqlx::query(r#"UPDATE stay_reservations SET "commissionAmount" = 20000.00, "operatorNetAmount" = 280000.00 WHERE id = $1"#)
        .bind(&id)
        .execute(&mut *tx)
        .await
        .unwrap();
    charge::sync_amended_commission(&mut tx, &id).await.unwrap();
    assert_eq!(balance(&mut tx).await, start - 2_000_000);
    assert_eq!(charged(&mut tx, &id).await, 2_000_000);
    tx.rollback().await.unwrap();
}
