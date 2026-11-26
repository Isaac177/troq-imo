use kernel::auth::{Actor, CityAccess, CurrentUser, Role};
use kernel::ids::UserId;
use kernel::permissions::Access;
use kernel::City;
use sqlx::postgres::PgPoolOptions;

use super::review;
use super::types::{ListingDecisionInput, ModerationInput};

#[tokio::test]
async fn moderation_writes_require_platform_permission_before_database_access() {
    let pool = PgPoolOptions::new().connect_lazy("postgres://localhost/mungini").unwrap();
    let actor = Actor {
        user: CurrentUser {
            user_id: UserId::new("seed-user"),
            role: Role::Host,
            issued_at: 0,
            auth_city: Some("GOMA".into()),
            city_access_mode: None,
        },
        city: City::GOMA,
        request_id: "seed-request".into(),
        city_access: CityAccess::Full,
        access: Access { grants: vec![], host_ids: vec![], auth_city: Some("GOMA".into()) },
    };
    let decision = ListingDecisionInput { decision: "APPROVE".into(), reason_codes: vec![], guidance: None, expected_version: 1 };
    let suspend = ModerationInput { expected_version: 1, reason_code: Some("SAFETY_CONCERN".into()) };
    let reinstate = ModerationInput { expected_version: 1, reason_code: None };
    assert_eq!(review::decide(&pool, &actor, "seed-revision", decision).await.unwrap_err().code().as_str(), "PERMISSION_MISSING");
    assert_eq!(review::suspend(&pool, &actor, "seed-property", suspend).await.unwrap_err().code().as_str(), "PERMISSION_MISSING");
    assert_eq!(review::reinstate(&pool, &actor, "seed-property", reinstate).await.unwrap_err().code().as_str(), "PERMISSION_MISSING");
}
