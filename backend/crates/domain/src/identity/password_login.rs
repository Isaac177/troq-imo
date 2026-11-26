use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;
use chrono::Utc;
use kernel::config::Config;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use sqlx::{PgPool, Postgres, Transaction};

use super::session_tokens::{self, TokenPair};
use super::sessions;

use super::messages::{ACCOUNT_BLOCKED, INVALID_CREDENTIALS};
const DUMMY_HASH: &str = "$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y";
const MAX_PASSWORD_BYTES: usize = 1024;

type LoginAccount = (String, String, Option<String>, Option<String>, String, Option<i64>);

fn verify(password: &str, hash: &str) -> bool {
    if hash.starts_with("$2") {
        return bcrypt::verify(password, hash).unwrap_or(false);
    }
    PasswordHash::new(hash)
        .ok()
        .is_some_and(|parsed| Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

async fn verify_off_thread(password: String, hash: String) -> Result<bool> {
    tokio::task::spawn_blocking(move || verify(&password, &hash))
        .await
        .map_err(|error| Error::internal(error.to_string()))
}

async fn record_attempt(tx: &mut Transaction<'_, Postgres>, user_id: &str, success: bool) -> Result<()> {
    sqlx::query("INSERT INTO identity_login_attempts (id, user_id, success) VALUES ($1, $2, $3)")
        .bind(new_id("login"))
        .bind(user_id)
        .bind(success)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn password_login(pool: &PgPool, config: &Config, identifier: &str, password: &str) -> Result<TokenPair> {
    let identifier = identifier.trim();
    let password = password.trim();
    if identifier.is_empty() || identifier.len() > 320 || password.is_empty() || password.len() > MAX_PASSWORD_BYTES {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let digits: String = identifier.chars().filter(char::is_ascii_digit).collect();
    let mut tx = pool.begin().await?;
    let accounts = sqlx::query_as::<_, LoginAccount>(
        r#"SELECT id, home_city_id, role, password_hash, status, revoked_before FROM identity_users
           WHERE ($2 AND lower(email) = lower($1))
              OR (NOT $2 AND $3 <> '' AND regexp_replace(COALESCE(phone, ''), '\D', '', 'g') = $3)
           ORDER BY id LIMIT 2 FOR UPDATE"#,
    )
    .bind(identifier)
    .bind(identifier.contains('@'))
    .bind(&digits)
    .fetch_all(&mut *tx)
    .await?;
    if accounts.len() != 1 {
        let _ = verify_off_thread(password.to_owned(), DUMMY_HASH.to_owned()).await?;
        return Err(Error::unauthorized(INVALID_CREDENTIALS));
    }
    let (user_id, city, role, hash, status, revoked_before) = accounts.into_iter().next().ok_or_else(|| Error::unauthorized(INVALID_CREDENTIALS))?;
    let failures: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM identity_login_attempts WHERE user_id = $1 AND success = false AND attempted_at >= now() - interval '15 minutes'",
    ).bind(&user_id).fetch_one(&mut *tx).await?;
    if failures >= 5 {
        return Err(Error::unauthorized(ACCOUNT_BLOCKED));
    }
    let valid = verify_off_thread(password.to_owned(), hash.clone().unwrap_or_else(|| DUMMY_HASH.to_owned())).await?;
    if !valid || hash.is_none() || status != "ACTIVE" {
        record_attempt(&mut tx, &user_id, false).await?;
        tx.commit().await?;
        return Err(Error::unauthorized(INVALID_CREDENTIALS));
    }
    let role = session_tokens::role(role)?;
    if revoked_before.is_some_and(|cutoff| Utc::now().timestamp() < cutoff) {
        return Err(Error::unauthorized(codes::SESSION_REVOKED));
    }
    kernel::City::from_code(&city).map_err(|_| Error::unauthorized(codes::AUTH_REQUIRED))?;
    let tokens = session_tokens::sign(&user_id, &city, role, &config.jwt_secret, &config.jwt_refresh_secret)?;
    sessions::store(&mut tx, &user_id, &tokens).await?;
    record_attempt(&mut tx, &user_id, true).await?;
    sqlx::query("UPDATE identity_users SET last_login_at = $2 WHERE id = $1")
        .bind(&user_id).bind(Utc::now()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(tokens.pair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::{PasswordHasher, SaltString};

    #[test]
    fn verifies_legacy_bcrypt_and_rejects_wrong_or_invalid_hashes() {
        assert!(verify("MunginiDemo2026!", DUMMY_HASH));
        assert!(!verify("incorrect", DUMMY_HASH));
        assert!(!verify("incorrect", "invalid"));
    }

    #[test]
    fn verifies_legacy_argon2_hashes() {
        let salt = SaltString::encode_b64(b"seed-test-salt").unwrap();
        let hash = Argon2::default().hash_password(b"example", &salt).unwrap().to_string();
        assert!(verify("example", &hash));
        assert!(!verify("wrong", &hash));
    }
}
