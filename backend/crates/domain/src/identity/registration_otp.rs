use hmac::{Hmac, Mac};
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use rand::{rngs::OsRng, Rng, RngCore};
use sha2::Sha256;
use sqlx::{PgPool, Postgres, Transaction};

use crate::notifications::{self, NotificationsConfig};
pub use crate::notifications::RegistrationChannel;

use super::messages::{CONTACT_EXISTS, CONTACT_INVALID, OTP_INVALID, OTP_RATE_LIMITED};

type OtpMac = Hmac<Sha256>;

fn channel_name(channel: RegistrationChannel) -> &'static str {
    match channel { RegistrationChannel::Email => "email", RegistrationChannel::Whatsapp => "whatsapp" }
}

fn destination(channel: RegistrationChannel, value: &str) -> Result<String> {
    let value = value.trim();
    match channel {
        RegistrationChannel::Email => {
            let email = value.to_ascii_lowercase();
            let valid = email.len() <= 254 && email.is_ascii() && !email.chars().any(|c| c.is_whitespace() || c.is_control())
                && email.split_once('@').is_some_and(|(local, domain)| {
                    !local.is_empty() && !domain.contains('@') && domain.contains('.')
                        && !domain.starts_with('.') && !domain.ends_with('.')
                });
            if valid { Ok(email) } else { Err(Error::bad_request(CONTACT_INVALID)) }
        }
        RegistrationChannel::Whatsapp => {
            let allowed = value.chars().all(|c| c.is_ascii_digit() || matches!(c, '+' | ' ' | '-' | '(' | ')'));
            let digits: String = value.chars().filter(char::is_ascii_digit).collect();
            if allowed && (8..=15).contains(&digits.len()) { Ok(digits) } else { Err(Error::bad_request(CONTACT_INVALID)) }
        }
    }
}

fn digest(key: &[u8], purpose: &str, channel: RegistrationChannel, destination: &str, value: &str) -> Result<Vec<u8>> {
    if key.len() < 32 { return Err(Error::internal("OTP signing key must be at least 32 bytes")); }
    let mut mac = OtpMac::new_from_slice(key).map_err(|_| Error::internal("invalid OTP signing key"))?;
    for part in [purpose, channel_name(channel), destination, value] {
        mac.update(&(part.len() as u64).to_be_bytes());
        mac.update(part.as_bytes());
    }
    Ok(mac.finalize().into_bytes().to_vec())
}

fn valid_digest(key: &[u8], purpose: &str, channel: RegistrationChannel, destination: &str, value: &str, expected: &[u8]) -> Result<bool> {
    if key.len() < 32 { return Err(Error::internal("OTP signing key must be at least 32 bytes")); }
    let mut mac = OtpMac::new_from_slice(key).map_err(|_| Error::internal("invalid OTP signing key"))?;
    for part in [purpose, channel_name(channel), destination, value] {
        mac.update(&(part.len() as u64).to_be_bytes());
        mac.update(part.as_bytes());
    }
    Ok(mac.verify_slice(expected).is_ok())
}

pub async fn issue_registration_otp(
    pool: &PgPool,
    config: &NotificationsConfig,
    key: &[u8],
    channel: RegistrationChannel,
    contact: &str,
    language: &str,
) -> Result<()> {
    let contact = destination(channel, contact)?;
    let exists: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM identity_users WHERE
            ($1 = 'email' AND lower(email) = $2) OR
            ($1 = 'whatsapp' AND regexp_replace(COALESCE(phone, ''), '\D', '', 'g') = $2))"#,
    ).bind(channel_name(channel)).bind(&contact).fetch_one(pool).await?;
    if exists { return Err(Error::conflict(CONTACT_EXISTS)); }
    let code = OsRng.gen_range(100_000..1_000_000).to_string();
    let hash = digest(key, "code", channel, &contact, &code)?;
    let id = new_id("otp");
    let stored: Option<String> = sqlx::query_scalar(
        "INSERT INTO identity_registration_otp (id, channel, destination, code_digest, expires_at) VALUES ($1, $2, $3, $4, now() + interval '15 minutes') ON CONFLICT (channel, destination) DO UPDATE SET id = EXCLUDED.id, code_digest = EXCLUDED.code_digest, attempts = 0, issued_at = now(), expires_at = EXCLUDED.expires_at, proof_digest = NULL, verified_until = NULL WHERE identity_registration_otp.issued_at <= now() - interval '1 minute' RETURNING id",
    ).bind(&id).bind(channel_name(channel)).bind(&contact).bind(hash).fetch_optional(pool).await?;
    if stored.is_none() { return Err(Error::conflict(OTP_RATE_LIMITED)); }
    let delivery_contact = match channel {
        RegistrationChannel::Email => contact.clone(),
        RegistrationChannel::Whatsapp => format!("+{contact}"),
    };
    if let Err(error) = notifications::send_registration_otp(config, channel, &delivery_contact, language, &code).await {
        sqlx::query("DELETE FROM identity_registration_otp WHERE id = $1 AND proof_digest IS NULL")
            .bind(&id).execute(pool).await?;
        return Err(error);
    }
    Ok(())
}

pub async fn verify_registration_otp(pool: &PgPool, key: &[u8], channel: RegistrationChannel, contact: &str, code: &str) -> Result<String> {
    let contact = destination(channel, contact)?;
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) { return Err(Error::bad_request(OTP_INVALID)); }
    let mut tx = pool.begin().await?;
    let row: Option<(String, Vec<u8>, i32, bool)> = sqlx::query_as(
        "SELECT id, code_digest, attempts, expires_at > now() AND proof_digest IS NULL FROM identity_registration_otp WHERE channel = $1 AND destination = $2 FOR UPDATE",
    ).bind(channel_name(channel)).bind(&contact).fetch_optional(&mut *tx).await?;
    let Some((id, hash, attempts, usable)) = row else { return Err(Error::bad_request(OTP_INVALID)); };
    if !usable || attempts >= 5 { return Err(Error::bad_request(OTP_INVALID)); }
    if !valid_digest(key, "code", channel, &contact, code, &hash)? {
        sqlx::query("UPDATE identity_registration_otp SET attempts = attempts + 1 WHERE id = $1")
            .bind(&id).execute(&mut *tx).await?;
        tx.commit().await?;
        return Err(Error::bad_request(OTP_INVALID));
    }
    let mut proof = [0u8; 32];
    OsRng.fill_bytes(&mut proof);
    let proof = hex::encode(proof);
    let proof_hash = digest(key, "proof", channel, &contact, &proof)?;
    sqlx::query("UPDATE identity_registration_otp SET proof_digest = $2, verified_until = now() + interval '30 minutes' WHERE id = $1")
        .bind(&id).bind(proof_hash).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(proof)
}

pub async fn consume_verification(
    tx: &mut Transaction<'_, Postgres>, key: &[u8], channel: RegistrationChannel, contact: &str, proof: &str,
) -> Result<bool> {
    let contact = destination(channel, contact)?;
    if proof.len() != 64 || !proof.chars().all(|c| c.is_ascii_hexdigit()) { return Ok(false); }
    let proof_hash = digest(key, "proof", channel, &contact, proof)?;
    let removed = sqlx::query("DELETE FROM identity_registration_otp WHERE channel = $1 AND destination = $2 AND proof_digest = $3 AND verified_until > now()")
        .bind(channel_name(channel)).bind(contact).bind(proof_hash).execute(&mut **tx).await?;
    Ok(removed.rows_affected() == 1)
}

pub async fn purge_expired_registration_challenges(pool: &PgPool, limit: i64) -> Result<u64> {
    if !(1..=1000).contains(&limit) {
        return Err(Error::bad_request(kernel::error::codes::PAGE_SIZE_INVALID));
    }
    let result = sqlx::query(
        "DELETE FROM identity_registration_otp WHERE id IN (SELECT id FROM identity_registration_otp WHERE COALESCE(verified_until, expires_at) <= now() ORDER BY COALESCE(verified_until, expires_at) LIMIT $1 FOR UPDATE SKIP LOCKED)",
    ).bind(limit).execute(pool).await?;
    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn verification_is_single_use_and_failed_attempts_are_bounded(pool: PgPool) {
        let key = b"test-otp-key-32-bytes-at-least-long";
        let email = "otp@example.com";
        let hash = digest(key, "code", RegistrationChannel::Email, email, "123456").unwrap();
        sqlx::query("INSERT INTO identity_registration_otp (id, channel, destination, code_digest, expires_at) VALUES ('seed-otp-test', 'email', $1, $2, now() + interval '15 minutes')")
            .bind(email).bind(hash).execute(&pool).await.unwrap();
        assert!(verify_registration_otp(&pool, key, RegistrationChannel::Email, email, "111111").await.is_err());
        let proof = verify_registration_otp(&pool, key, RegistrationChannel::Email, email, "123456").await.unwrap();
        assert!(verify_registration_otp(&pool, key, RegistrationChannel::Email, email, "123456").await.is_err());
        let mut tx = pool.begin().await.unwrap();
        assert!(consume_verification(&mut tx, key, RegistrationChannel::Email, email, &proof).await.unwrap());
        assert!(!consume_verification(&mut tx, key, RegistrationChannel::Email, email, &proof).await.unwrap());
        tx.commit().await.unwrap();
        let hash = digest(key, "code", RegistrationChannel::Email, email, "123456").unwrap();
        sqlx::query("INSERT INTO identity_registration_otp (id, channel, destination, code_digest, expires_at) VALUES ('seed-otp-test-locked', 'email', $1, $2, now() + interval '15 minutes')")
            .bind(email).bind(hash).execute(&pool).await.unwrap();
        for _ in 0..5 {
            assert!(verify_registration_otp(&pool, key, RegistrationChannel::Email, email, "111111").await.is_err());
        }
        assert!(verify_registration_otp(&pool, key, RegistrationChannel::Email, email, "123456").await.is_err());
    }

    #[test]
    fn contacts_are_normalized_and_invalid_contacts_rejected() {
        assert_eq!(destination(RegistrationChannel::Email, " User@Example.com ").unwrap(), "user@example.com");
        assert_eq!(destination(RegistrationChannel::Whatsapp, "+243 810 000 001").unwrap(), "243810000001");
        assert!(destination(RegistrationChannel::Email, "bad@example").is_err());
        assert!(destination(RegistrationChannel::Whatsapp, "123").is_err());
    }

    #[test]
    fn otp_hashes_bind_the_contact_channel_and_purpose() {
        let key = b"test-otp-key-32-bytes-at-least-long";
        let hash = digest(key, "code", RegistrationChannel::Email, "a@example.com", "123456").unwrap();
        assert!(valid_digest(key, "code", RegistrationChannel::Email, "a@example.com", "123456", &hash).unwrap());
        assert!(!valid_digest(key, "proof", RegistrationChannel::Email, "a@example.com", "123456", &hash).unwrap());
        assert!(!valid_digest(key, "code", RegistrationChannel::Email, "b@example.com", "123456", &hash).unwrap());
        assert!(digest(b"short", "code", RegistrationChannel::Email, "a@example.com", "123456").is_err());
    }
}
