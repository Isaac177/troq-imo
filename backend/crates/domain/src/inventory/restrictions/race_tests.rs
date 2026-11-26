use kernel::ids::new_id;

use super::{apply_bulk_edit, BulkRestrictionInput, RestrictionOperation};
use crate::inventory::test_support::{goma_property, staff_actor, test_pool, GOMA_PROPERTY, STAFF_USER};

fn bulk_add(kind: &str, value: i32) -> RestrictionOperation {
    RestrictionOperation {
        action: "ADD".into(),
        restriction_id: None,
        kind: Some(kind.into()),
        basis: None,
        unit_type_id: None,
        start_date: None,
        end_date: None,
        value: Some(value),
    }
}

fn bulk_remove(restriction_id: String) -> RestrictionOperation {
    RestrictionOperation {
        action: "REMOVE".into(),
        restriction_id: Some(restriction_id),
        kind: None,
        basis: None,
        unit_type_id: None,
        start_date: None,
        end_date: None,
        value: None,
    }
}

#[tokio::test]
#[ignore]
async fn competing_removals_of_one_restriction_apply_exactly_once() {
    let pool = test_pool().await;
    let restriction = new_id("race-restr");
    sqlx::query(
        r#"INSERT INTO stay_restrictions (id, city_id, "propertyId", kind, basis, value, "actorUserId")
           VALUES ($1, 'GOMA', $2, 'MAX_NIGHTS', 'STAY_THROUGH', 40, $3)"#,
    )
    .bind(&restriction)
    .bind(GOMA_PROPERTY)
    .bind(STAFF_USER)
    .execute(&pool)
    .await
    .unwrap();
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let pool = pool.clone();
        let restriction = restriction.clone();
        tasks.push(tokio::spawn(async move {
            let input = BulkRestrictionInput {
                property_id: GOMA_PROPERTY.into(),
                operations: vec![bulk_remove(restriction)],
                idempotency_key: None,
            };
            apply_bulk_edit(&pool, &goma_property(), &staff_actor(), input).await.is_ok()
        }));
    }
    let mut winners = 0;
    for task in tasks {
        if task.await.unwrap() {
            winners += 1;
        }
    }
    let (version, removed): (i32, bool) = sqlx::query_as(
        r#"SELECT version, "removedAt" IS NOT NULL FROM stay_restrictions WHERE id = $1"#,
    )
    .bind(&restriction)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(winners, 1);
    assert!(removed);
    assert_eq!(version, 2, "the removal must bump the version exactly once");
}

#[tokio::test]
#[ignore]
async fn competing_keyed_bulk_edits_apply_once() {
    let pool = test_pool().await;
    let key = new_id("race-key");
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let pool = pool.clone();
        let key = key.clone();
        tasks.push(tokio::spawn(async move {
            let input = BulkRestrictionInput {
                property_id: GOMA_PROPERTY.into(),
                operations: vec![bulk_add("ADVANCE_NOTICE_DAYS", 91)],
                idempotency_key: Some(key),
            };
            apply_bulk_edit(&pool, &goma_property(), &staff_actor(), input).await.is_ok()
        }));
    }
    let mut succeeded = 0;
    for task in tasks {
        if task.await.unwrap() {
            succeeded += 1;
        }
    }
    let rows: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_restrictions
           WHERE "propertyId" = $1 AND kind = 'ADVANCE_NOTICE_DAYS' AND value = 91"#,
    )
    .bind(GOMA_PROPERTY)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"DELETE FROM stay_restrictions WHERE "propertyId" = $1 AND kind = 'ADVANCE_NOTICE_DAYS' AND value = 91"#,
    )
    .bind(GOMA_PROPERTY)
    .execute(&pool)
    .await
    .unwrap();
    assert!(succeeded >= 1);
    assert_eq!(rows, 1, "a keyed bulk edit must create its restriction exactly once");
}
