use kernel::auth::{directory::is_revoked, CurrentUser};
use kernel::config::Config;
use kernel::error::{codes, Error, Result};
use sqlx::{PgPool, Postgres, Transaction};

use super::session_tokens::{self, SignedTokens, TokenPair};

type Account = (Option<String>, String, Option<i64>);

async fn lock_account(tx: &mut Transaction<'_, Postgres>, user_id: &str) -> Result<Account> {
    sqlx::query_as::<_, Account>(
        "SELECT role, home_city_id, revoked_before FROM identity_users WHERE id = $1 AND status = 'ACTIVE' FOR UPDATE",
    )
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| Error::unauthorized(codes::AUTH_REQUIRED))
}

pub(super) async fn store(tx: &mut Transaction<'_, Postgres>, user_id: &str, tokens: &SignedTokens) -> Result<()> {
    sqlx::query("INSERT INTO identity_refresh_sessions (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)")
        .bind(&tokens.session_id)
        .bind(user_id)
        .bind(&tokens.token_hash)
        .bind(tokens.expires_at)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn issue_session(pool: &PgPool, config: &Config, user: &CurrentUser) -> Result<TokenPair> {
    let mut tx = pool.begin().await?;
    let (stored_role, city, revoked_before) = lock_account(&mut tx, user.user_id.as_str()).await?;
    let role = session_tokens::role(stored_role)?;
    if role != user.role || is_revoked(revoked_before, user.issued_at) {
        return Err(Error::unauthorized(codes::SESSION_REVOKED));
    }
    kernel::City::from_code(&city).map_err(|_| Error::unauthorized(codes::AUTH_REQUIRED))?;
    let tokens = session_tokens::sign(user.user_id.as_str(), &city, role, &config.jwt_secret, &config.jwt_refresh_secret)?;
    store(&mut tx, user.user_id.as_str(), &tokens).await?;
    tx.commit().await?;
    Ok(tokens.pair)
}

pub async fn refresh_session(pool: &PgPool, config: &Config, token: &str) -> Result<TokenPair> {
    let (user_id, session_id, issued_at) = session_tokens::verify_refresh(token, &config.jwt_refresh_secret)?;
    let mut tx = pool.begin().await?;
    let (stored_role, city, revoked_before) = lock_account(&mut tx, &user_id).await?;
    if is_revoked(revoked_before, issued_at) {
        return Err(Error::unauthorized(codes::SESSION_REVOKED));
    }
    let role = session_tokens::role(stored_role)?;
    kernel::City::from_code(&city).map_err(|_| Error::unauthorized(codes::AUTH_REQUIRED))?;
    let found = sqlx::query_scalar::<_, String>(
        "SELECT id FROM identity_refresh_sessions WHERE id = $1 AND user_id = $2 AND token_hash = $3 AND expires_at > now() AND (revoked_at IS NULL OR revoked_at >= now() - interval '30 seconds') FOR UPDATE",
    )
    .bind(&session_id)
    .bind(&user_id)
    .bind(session_tokens::hash(token))
    .fetch_optional(&mut *tx)
    .await?;
    if found.is_none() {
        return Err(Error::unauthorized(codes::TOKEN_INVALID));
    }
    sqlx::query("UPDATE identity_refresh_sessions SET revoked_at = COALESCE(revoked_at, now()) WHERE id = $1")
        .bind(&session_id).execute(&mut *tx).await?;
    let tokens = session_tokens::sign(&user_id, &city, role, &config.jwt_secret, &config.jwt_refresh_secret)?;
    store(&mut tx, &user_id, &tokens).await?;
    tx.commit().await?;
    Ok(tokens.pair)
}

pub async fn logout_all_sessions(pool: &PgPool, user: &CurrentUser) -> Result<()> {
    let mut tx = pool.begin().await?;
    let (_, _, cutoff) = lock_account(&mut tx, user.user_id.as_str()).await?;
    if is_revoked(cutoff, user.issued_at) {
        return Err(Error::unauthorized(codes::SESSION_REVOKED));
    }
    sqlx::query("UPDATE identity_users SET revoked_before = GREATEST(COALESCE(revoked_before, 0), FLOOR(EXTRACT(EPOCH FROM now()))::bigint + 1) WHERE id = $1")
        .bind(user.user_id.as_str()).execute(&mut *tx).await?;
    sqlx::query("UPDATE identity_refresh_sessions SET revoked_at = COALESCE(revoked_at, now()) WHERE user_id = $1")
        .bind(user.user_id.as_str()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
