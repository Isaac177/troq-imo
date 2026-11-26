use std::time::Duration;

use kernel::auth::{Actor, CityAccess, CurrentUser, Role};
use kernel::city::City;
use kernel::ids::{new_id, HostId, UserId};
use kernel::permissions::names::{PROPERTIES_MODERATE, PROPERTIES_WRITE};
use kernel::permissions::{Access, Grant};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use super::types::{ListingDecisionInput, LifecycleInput, ModerationInput};
use super::{lifecycle, review};
use crate::properties::property::queries::find;

const HOST_USER: &str = "seed-user-goma-host";
const HOST: &str = "seed-host-goma-lake";
const LAKESIDE: &str = "seed-prop-goma-lakeside";
const SUBMITTED_REVISION: &str = "seed-revision-goma-submitted";

async fn test_pool() -> PgPool {
    let url = std::env::var("MUNGINI_TEST_DATABASE_URL")
        .expect("set MUNGINI_TEST_DATABASE_URL to a database with the Goma seed loaded");
    PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&url)
        .await
        .expect("connect to test database")
}

fn actor(user: &str, role: Role, grants: Vec<Grant>) -> Actor {
    Actor {
        user: CurrentUser {
            user_id: UserId::new(user),
            role,
            issued_at: 0,
            auth_city: None,
            city_access_mode: None,
        },
        city: City::GOMA,
        request_id: new_id("req"),
        city_access: CityAccess::Full,
        access: Access { grants, host_ids: vec![HostId::new(HOST)], auth_city: None },
    }
}

fn moderator() -> Actor {
    actor("seed-user-goma-admin", Role::Staff, vec![Grant::platform(PROPERTIES_MODERATE)])
}

fn host() -> Actor {
    actor(HOST_USER, Role::Host, vec![Grant::host(PROPERTIES_WRITE, HOST)])
}

#[tokio::test]
#[ignore]
async fn a_stale_decision_is_refused_before_anything_is_applied() {
    let pool = test_pool().await;
    let before = find(&pool, "seed-prop-goma-ready").await.unwrap().unwrap();
    let result = review::decide(
        &pool,
        &moderator(),
        SUBMITTED_REVISION,
        ListingDecisionInput { decision: "APPROVE".into(), reason_codes: Vec::new(), guidance: None, expected_version: 99 },
    )
    .await;
    assert_eq!(result.unwrap_err().code().as_str(), "LISTING_VERSION_CONFLICT");
    let after = find(&pool, "seed-prop-goma-ready").await.unwrap().unwrap();
    assert_eq!(before.published_revision, after.published_revision);
    assert_eq!(before.version, after.version);
}

#[tokio::test]
#[ignore]
async fn suspension_needs_a_reason_and_the_current_version() {
    let pool = test_pool().await;
    let missing = review::suspend(&pool, &moderator(), LAKESIDE, ModerationInput { expected_version: 1, reason_code: None }).await;
    assert_eq!(missing.unwrap_err().code().as_str(), "REASON_CODE_REQUIRED");
    let stale = review::suspend(
        &pool,
        &moderator(),
        LAKESIDE,
        ModerationInput { expected_version: 99, reason_code: Some("OTHER".into()) },
    )
    .await;
    assert_eq!(stale.unwrap_err().code().as_str(), "LISTING_VERSION_CONFLICT");
}

#[tokio::test]
#[ignore]
async fn only_a_suspended_property_can_be_reinstated() {
    let pool = test_pool().await;
    let property = find(&pool, LAKESIDE).await.unwrap().unwrap();
    let result = review::reinstate(
        &pool,
        &moderator(),
        LAKESIDE,
        ModerationInput { expected_version: property.version, reason_code: None },
    )
    .await;
    assert_eq!(result.unwrap_err().code().as_str(), "LISTING_STATUS_INVALID");
}

#[tokio::test]
#[ignore]
async fn archiving_is_refused_while_a_stay_is_upcoming() {
    let pool = test_pool().await;
    let property = find(&pool, LAKESIDE).await.unwrap().unwrap();
    let result = lifecycle::apply(
        &pool,
        &host(),
        LAKESIDE,
        LifecycleInput { action: "ARCHIVE".into(), expected_version: property.version },
        false,
    )
    .await;
    assert_eq!(result.unwrap_err().code().as_str(), "LISTING_UPCOMING_STAYS");
}

#[tokio::test]
#[ignore]
async fn unknown_lifecycle_actions_are_rejected() {
    let pool = test_pool().await;
    let property = find(&pool, LAKESIDE).await.unwrap().unwrap();
    let result = lifecycle::apply(
        &pool,
        &host(),
        LAKESIDE,
        LifecycleInput { action: "EXPLODE".into(), expected_version: property.version },
        false,
    )
    .await;
    assert_eq!(result.unwrap_err().code().as_str(), "LISTING_ACTION_UNKNOWN");
}
