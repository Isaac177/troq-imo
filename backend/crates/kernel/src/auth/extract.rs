use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::city::City;
use crate::error::codes::CITY_VIEW_ONLY;
use crate::error::{Error, Result};
use crate::ids::UserId;
use crate::kernel::{Kernel, KernelState};

use super::actor::{city_access_for, Actor, CityAccess, CurrentUser};
use super::directory::is_revoked;
use super::request::{
    bearer_token, is_write_method, request_id, resolve_city, session_revoked,
};

pub struct RequestCity(pub City);

pub struct RequestId(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for RequestCity
where
    S: KernelState + Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        resolve_city(parts, state.kernel()).map(|(city, _)| Self(city))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        Ok(Self(request_id(parts)))
    }
}

async fn authenticate(parts: &Parts, kernel: &Kernel) -> Result<CurrentUser> {
    let claims = kernel.tokens.verify(bearer_token(parts)?)?;
    let user_id = UserId::new(claims.sub);
    let revoked_before = kernel.directory.revoked_before(&user_id).await?;
    if is_revoked(revoked_before, claims.iat) {
        return Err(session_revoked());
    }
    Ok(CurrentUser {
        user_id,
        role: claims.role,
        issued_at: claims.iat,
        auth_city: claims.auth_city,
        city_access_mode: claims.city_access_mode,
    })
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: KernelState + Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        authenticate(parts, state.kernel()).await
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Actor
where
    S: KernelState + Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let kernel = state.kernel();
        let request_id = request_id(parts);
        let (city, explicit) = resolve_city(parts, kernel)?;
        let user = authenticate(parts, kernel).await?;
        let access = kernel.directory.access_for(&user.user_id).await?;
        let auth_city = access
            .auth_city
            .as_deref()
            .or(user.auth_city.as_deref())
            .and_then(|code| City::from_code(code).ok());
        let city_access = city_access_for(
            explicit.then_some(city),
            auth_city,
            user.city_access_mode.as_deref(),
        );
        if city_access == CityAccess::ViewOnly && is_write_method(parts) {
            return Err(Error::forbidden(CITY_VIEW_ONLY));
        }
        Ok(Actor {
            user,
            city,
            request_id,
            city_access,
            access,
        })
    }
}
