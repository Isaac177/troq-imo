use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::error::codes::TOKEN_INVALID;
use crate::error::{Error, Result};

pub const ACCESS_TOKEN_TYPE: &str = "access";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Guest,
    Host,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    #[serde(rename = "type")]
    pub token_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_access_mode: Option<String>,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct TokenVerifier {
    key: DecodingKey,
    validation: Validation,
}

impl TokenVerifier {
    pub fn new(secret: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0;
        Self {
            key: DecodingKey::from_secret(secret.as_bytes()),
            validation,
        }
    }

    pub fn verify(&self, token: &str) -> Result<Claims> {
        let claims = decode::<Claims>(token, &self.key, &self.validation)
            .map_err(|_| Error::unauthorized(TOKEN_INVALID))?
            .claims;
        if claims.token_type != ACCESS_TOKEN_TYPE || claims.sub.trim().is_empty() {
            return Err(Error::unauthorized(TOKEN_INVALID));
        }
        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    fn claims(token_type: &str, exp_offset: i64) -> Claims {
        let now = chrono::Utc::now().timestamp();
        Claims {
            sub: "user-1".into(),
            role: Role::Host,
            token_type: token_type.into(),
            auth_city: Some("GOMA".into()),
            city_access_mode: None,
            exp: now + exp_offset,
            iat: now,
        }
    }

    fn sign(claims: &Claims, secret: &str) -> String {
        encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
    }

    #[test]
    fn accepts_a_valid_access_token() {
        let token = sign(&claims("access", 600), "s");
        let verified = TokenVerifier::new("s").verify(&token).unwrap();
        assert_eq!(verified.sub, "user-1");
        assert_eq!(verified.role, Role::Host);
    }

    #[test]
    fn rejects_wrong_secret_expired_and_refresh_tokens() {
        let verifier = TokenVerifier::new("s");
        assert!(verifier.verify(&sign(&claims("access", 600), "other")).is_err());
        assert!(verifier.verify(&sign(&claims("access", -10), "s")).is_err());
        assert!(verifier.verify(&sign(&claims("refresh", 600), "s")).is_err());
        assert!(verifier.verify("garbage").is_err());
    }
}
