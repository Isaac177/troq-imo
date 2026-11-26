use std::time::Duration;

use kernel::auth::{CurrentUser, Role, TokenVerifier};
use kernel::config::{Config, OtpChannel};
use kernel::ids::UserId;
use kernel::storage::StorageConfig;
use kernel::City;
use sqlx::PgPool;

use super::sessions::{issue_session, logout_all_sessions, refresh_session};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

fn config() -> Config {
    Config {
        production: false,
        api_port: 0,
        jobs_enabled: false,
        database_url: String::new(),
        redis_url: String::new(),
        cors_allowed_origins: vec![],
        cities: vec![City::GOMA],
        default_city: City::GOMA,
        jwt_secret: "access-secret".into(),
        jwt_refresh_secret: "refresh-secret".into(),
        otp_signing_key: "test-otp-key-32-bytes-at-least-long".into(),
        otp_channel: OtpChannel::Email,
        guest_retention_days: None,
        storage: StorageConfig {
            endpoint: String::new(), signing_endpoint: None, region: String::new(),
            access_key_id: String::new(), secret_access_key: String::new(), bucket_public: String::new(),
            bucket_private: String::new(), public_base_url: String::new(), force_path_style: true,
            signed_url_ttl: Duration::from_secs(300), upload_timeout: Duration::from_secs(20), upload_retries: 3,
        },
    }
}

fn user() -> CurrentUser {
    CurrentUser {
        user_id: UserId::new("seed-session-user"),
        role: Role::Host,
        issued_at: chrono::Utc::now().timestamp(),
        auth_city: Some("GOMA".into()),
        city_access_mode: None,
    }
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn rotates_refresh_and_logout_revokes_the_whole_session(pool: PgPool) {
    sqlx::query("INSERT INTO identity_users (id, home_city_id, status, role) VALUES ('seed-session-user', 'GOMA', 'ACTIVE', 'host')")
        .execute(&pool).await.unwrap();
    let config = config();
    let current_user = user();
    let original = issue_session(&pool, &config, &current_user).await.unwrap();
    let claims = TokenVerifier::new(&config.jwt_secret).verify(&original.access_token).unwrap();
    assert_eq!(claims.sub, "seed-session-user");
    assert_eq!(claims.role, Role::Host);
    let rotated = refresh_session(&pool, &config, &original.refresh_token).await.unwrap();
    assert_ne!(original.refresh_token, rotated.refresh_token);
    assert!(refresh_session(&pool, &config, &original.refresh_token).await.is_ok());
    logout_all_sessions(&pool, &current_user).await.unwrap();
    assert!(refresh_session(&pool, &config, &rotated.refresh_token).await.is_err());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn refuses_suspended_unassigned_and_role_mismatched_accounts(pool: PgPool) {
    sqlx::query("INSERT INTO identity_users (id, home_city_id, status, role) VALUES ('seed-session-user', 'GOMA', 'SUSPENDED', 'host')")
        .execute(&pool).await.unwrap();
    assert!(issue_session(&pool, &config(), &user()).await.is_err());
    sqlx::query("UPDATE identity_users SET status = 'ACTIVE', role = NULL WHERE id = 'seed-session-user'")
        .execute(&pool).await.unwrap();
    assert!(issue_session(&pool, &config(), &user()).await.is_err());
    sqlx::query("UPDATE identity_users SET role = 'guest' WHERE id = 'seed-session-user'")
        .execute(&pool).await.unwrap();
    assert!(issue_session(&pool, &config(), &user()).await.is_err());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn password_login_locks_out_and_creates_a_session_after_expiry(pool: PgPool) {
    sqlx::query("INSERT INTO identity_users (id, home_city_id, status, role, email, password_hash) VALUES ($1, 'GOMA', 'ACTIVE', 'host', $2, $3)")
        .bind("seed-password-user").bind("host@example.com")
        .bind("$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y")
        .execute(&pool).await.unwrap();
    for _ in 0..5 {
        assert_eq!(super::password_login(&pool, &config(), "HOST@example.com", "wrong").await.unwrap_err().code().as_str(), "INVALID_CREDENTIALS");
    }
    assert_eq!(super::password_login(&pool, &config(), "host@example.com", "MunginiDemo2026!").await.unwrap_err().code().as_str(), "ACCOUNT_BLOCKED");
    sqlx::query("UPDATE identity_login_attempts SET attempted_at = now() - interval '16 minutes' WHERE user_id = $1")
        .bind("seed-password-user").execute(&pool).await.unwrap();
    let pair = super::password_login(&pool, &config(), "host@example.com", "MunginiDemo2026!").await.unwrap();
    assert_eq!(TokenVerifier::new(&config().jwt_secret).verify(&pair.access_token).unwrap().sub, "seed-password-user");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM identity_refresh_sessions WHERE user_id = $1")
        .bind("seed-password-user").fetch_one(&pool).await.unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn password_login_rejects_ambiguous_identifiers(pool: PgPool) {
    sqlx::query("INSERT INTO identity_users (id, home_city_id, status, role, email, password_hash) VALUES ($1, 'GOMA', 'ACTIVE', 'host', $2, $3), ($4, 'KIN', 'ACTIVE', 'host', $2, $3)")
        .bind("seed-password-a").bind("same@example.com")
        .bind("$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y")
        .bind("seed-password-b").execute(&pool).await.unwrap();
    assert_eq!(super::password_login(&pool, &config(), "same@example.com", "MunginiDemo2026!").await.unwrap_err().code().as_str(), "INVALID_CREDENTIALS");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM identity_refresh_sessions")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count, 0);
}
