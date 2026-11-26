use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use chrono::Utc;
use kernel::auth::Role;
use kernel::city::City;
use kernel::config::Config;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use sqlx::{PgPool, Postgres, Transaction};

use super::registration_otp::{self, RegistrationChannel};
use super::session_tokens::{self, TokenPair};
use super::sessions;

const MAX_NAME_CHARS: usize = 80;
const MAX_PASSWORD_BYTES: usize = 1024;

#[derive(Debug, Clone)]
pub struct RegisterInput {
    pub channel: RegistrationChannel,
    pub contact: String,
    pub proof: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: City,
    pub preferred_language: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileLanguage {
    En,
    Fr,
}

impl ProfileLanguage {
    pub fn parse(value: &str) -> Result<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            _ => Err(Error::bad_request(codes::MALFORMED_REQUEST)),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
        }
    }
}

fn name(value: Option<String>) -> Result<Option<String>> {
    let value = value.unwrap_or_default().trim().to_string();
    if value.chars().count() > MAX_NAME_CHARS {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    Ok((!value.is_empty()).then_some(value))
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| Error::internal(error.to_string()))
}

async fn contact_taken(pool: &PgPool, channel: RegistrationChannel, contact: &str) -> Result<bool> {
    let taken: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM identity_users WHERE
            ($1 = 'email' AND lower(email) = $2) OR
            ($1 = 'whatsapp' AND regexp_replace(COALESCE(phone, ''), '\D', '', 'g') = $2))"#,
    )
    .bind(match channel {
        RegistrationChannel::Email => "email",
        RegistrationChannel::Whatsapp => "whatsapp",
    })
    .bind(contact)
    .fetch_one(pool)
    .await?;
    Ok(taken)
}

fn channel_name(channel: RegistrationChannel) -> &'static str {
    match channel {
        RegistrationChannel::Email => "email",
        RegistrationChannel::Whatsapp => "whatsapp",
    }
}

async fn contact_taken_tx(
    tx: &mut Transaction<'_, Postgres>,
    channel: RegistrationChannel,
    contact: &str,
) -> Result<bool> {
    let taken: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM identity_users WHERE
            ($1 = 'email' AND lower(email) = $2) OR
            ($1 = 'whatsapp' AND regexp_replace(COALESCE(phone, ''), '\D', '', 'g') = $2))"#,
    )
    .bind(channel_name(channel))
    .bind(contact)
    .fetch_one(&mut **tx)
    .await?;
    Ok(taken)
}

pub async fn register(
    pool: &PgPool,
    config: &Config,
    otp_key: &[u8],
    input: RegisterInput,
) -> Result<TokenPair> {
    let password = input.password.trim();
    if password.is_empty() || password.len() > MAX_PASSWORD_BYTES {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let first_name = name(input.first_name)?;
    let last_name = name(input.last_name)?;
    let language = ProfileLanguage::parse(&input.preferred_language)?;
    let contact = match input.channel {
        RegistrationChannel::Email => input.contact.trim().to_ascii_lowercase(),
        RegistrationChannel::Whatsapp => input.contact.chars().filter(char::is_ascii_digit).collect(),
    };
    if contact_taken(pool, input.channel, &contact).await? {
        return Err(Error::conflict(super::messages::CONTACT_EXISTS));
    }
    let password_hash = hash_password(password)?;
    let user_id = new_id("user");
    let verified_at = Utc::now();
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext('identity_user_contact'))")
        .execute(&mut *tx)
        .await?;
    if contact_taken_tx(&mut tx, input.channel, &contact).await? {
        return Err(Error::conflict(super::messages::CONTACT_EXISTS));
    }
    let verified = registration_otp::consume_verification(
        &mut tx,
        otp_key,
        input.channel,
        &contact,
        &input.proof,
    )
    .await?;
    if !verified {
        return Err(Error::bad_request(super::messages::OTP_INVALID));
    }
    let (email, phone, email_verified_at) = match input.channel {
        RegistrationChannel::Email => (Some(contact.as_str()), None, Some(verified_at)),
        RegistrationChannel::Whatsapp => (None, Some(contact.as_str()), None),
    };
    sqlx::query(
        "INSERT INTO identity_users (id, home_city_id, status, role, email, phone, preferred_language, password_hash, first_name, last_name, email_verified_at) VALUES ($1, $2, 'ACTIVE', 'guest', $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(&user_id)
    .bind(input.city.code())
    .bind(email)
    .bind(phone)
    .bind(language.as_str())
    .bind(&password_hash)
    .bind(&first_name)
    .bind(&last_name)
    .bind(email_verified_at)
    .execute(&mut *tx)
    .await?;
    let tokens = session_tokens::sign(&user_id, input.city.code(), Role::Guest, &config.jwt_secret, &config.jwt_refresh_secret)?;
    sessions::store(&mut tx, &user_id, &tokens).await?;
    tx.commit().await?;
    Ok(tokens.pair)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_language: Option<String>,
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: &str,
    input: UpdateProfileInput,
) -> Result<()> {
    let first_name = name(input.first_name)?;
    let last_name = name(input.last_name)?;
    let language = input
        .preferred_language
        .as_deref()
        .map(ProfileLanguage::parse)
        .transpose()?;
    let updated = sqlx::query(
        "UPDATE identity_users SET first_name = COALESCE($2, first_name), last_name = COALESCE($3, last_name), preferred_language = COALESCE($4, preferred_language) WHERE id = $1 AND status = 'ACTIVE'",
    )
    .bind(user_id)
    .bind(first_name)
    .bind(last_name)
    .bind(language.map(ProfileLanguage::as_str))
    .execute(pool)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(Error::unauthorized(codes::AUTH_REQUIRED));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_language_accepts_only_english_and_french() {
        assert_eq!(ProfileLanguage::parse("EN").unwrap(), ProfileLanguage::En);
        assert_eq!(ProfileLanguage::parse("fr").unwrap().as_str(), "fr");
        assert!(ProfileLanguage::parse("es").is_err());
    }

    #[test]
    fn names_are_trimmed_and_bounded() {
        assert_eq!(name(Some("  Ada  ".into())).unwrap().as_deref(), Some("Ada"));
        assert!(name(Some("x".repeat(MAX_NAME_CHARS + 1))).is_err());
    }

    #[test]
    fn password_hashes_use_argon2_and_verify() {
        use argon2::password_hash::PasswordVerifier;
        let hash = hash_password("secret").unwrap();
        let parsed = argon2::password_hash::PasswordHash::new(&hash).unwrap();
        assert!(Argon2::default().verify_password(b"secret", &parsed).is_ok());
    }
}
