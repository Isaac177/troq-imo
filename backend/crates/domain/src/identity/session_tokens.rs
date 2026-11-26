use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use kernel::auth::{Claims, Role};
use kernel::error::{codes, Error, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const ACCESS_SECONDS: i64 = 15 * 60;
const REFRESH_DAYS: i64 = 7;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshClaims {
    sub: String,
    #[serde(rename = "type")]
    token_type: String,
    jti: String,
    iat: i64,
    exp: i64,
}

pub(super) struct SignedTokens {
    pub pair: TokenPair,
    pub token_hash: Vec<u8>,
    pub expires_at: chrono::DateTime<Utc>,
    pub session_id: String,
}

pub(super) fn sign(user_id: &str, city: &str, role: Role, access_secret: &str, refresh_secret: &str) -> Result<SignedTokens> {
    let now = Utc::now();
    let expires_at = now + Duration::days(REFRESH_DAYS);
    let session_id = kernel::ids::new_id("session");
    let access = Claims {
        sub: user_id.to_owned(), role, token_type: "access".into(), auth_city: Some(city.to_owned()),
        city_access_mode: None, iat: now.timestamp(), exp: (now + Duration::seconds(ACCESS_SECONDS)).timestamp(),
    };
    let refresh = RefreshClaims {
        sub: user_id.to_owned(), token_type: "refresh".into(), jti: session_id.clone(),
        iat: now.timestamp(), exp: expires_at.timestamp(),
    };
    let header = Header::new(Algorithm::HS256);
    let access_token = encode(&header, &access, &EncodingKey::from_secret(access_secret.as_bytes()))
        .map_err(|error| Error::internal(error.to_string()))?;
    let refresh_token = encode(&header, &refresh, &EncodingKey::from_secret(refresh_secret.as_bytes()))
        .map_err(|error| Error::internal(error.to_string()))?;
    let token_hash = hash(&refresh_token);
    Ok(SignedTokens { pair: TokenPair { access_token, refresh_token }, token_hash, expires_at, session_id })
}

pub(super) fn verify_refresh(token: &str, secret: &str) -> Result<(String, String, i64)> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0;
    let claims = decode::<RefreshClaims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
        .map_err(|_| Error::unauthorized(codes::TOKEN_INVALID))?.claims;
    if claims.token_type != "refresh" || claims.sub.trim().is_empty() || claims.jti.trim().is_empty() {
        return Err(Error::unauthorized(codes::TOKEN_INVALID));
    }
    Ok((claims.sub, claims.jti, claims.iat))
}

pub(super) fn hash(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

pub(super) fn role(value: Option<String>) -> Result<Role> {
    match value.as_deref() {
        Some("guest") => Ok(Role::Guest),
        Some("host") => Ok(Role::Host),
        Some("staff") => Ok(Role::Staff),
        _ => Err(Error::unauthorized(codes::AUTH_REQUIRED)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::auth::TokenVerifier;

    #[test]
    fn access_and_refresh_tokens_use_distinct_keys_and_types() {
        let tokens = sign("seed-user", "GOMA", Role::Host, "access-secret", "refresh-secret").unwrap();
        assert!(TokenVerifier::new("access-secret").verify(&tokens.pair.access_token).is_ok());
        assert!(TokenVerifier::new("access-secret").verify(&tokens.pair.refresh_token).is_err());
        assert_eq!(verify_refresh(&tokens.pair.refresh_token, "refresh-secret").unwrap().0, "seed-user");
        assert!(verify_refresh(&tokens.pair.refresh_token, "other").is_err());
        assert_eq!(tokens.token_hash, hash(&tokens.pair.refresh_token));
    }

    #[test]
    fn only_explicit_known_account_roles_are_accepted() {
        assert!(role(None).is_err());
        assert!(role(Some("admin".into())).is_err());
        assert_eq!(role(Some("host".into())).unwrap(), Role::Host);
    }
}
