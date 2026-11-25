use crate::audit::AuditActor;
use crate::city::City;
use crate::error::codes::CITY_VIEW_ONLY;
use crate::error::{Error, Result};
use crate::ids::{HostId, UserId};
use crate::permissions::Access;

use super::claims::Role;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CityAccess {
    Full,
    ViewOnly,
}

pub fn city_access_for(
    requested: Option<City>,
    auth_city: Option<City>,
    token_mode: Option<&str>,
) -> CityAccess {
    match (requested, auth_city) {
        (Some(requested), Some(auth_city)) if requested != auth_city => CityAccess::ViewOnly,
        (Some(_), Some(_)) => CityAccess::Full,
        _ if token_mode == Some("view_only") => CityAccess::ViewOnly,
        _ => CityAccess::Full,
    }
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: UserId,
    pub role: Role,
    pub issued_at: i64,
    pub auth_city: Option<String>,
    pub city_access_mode: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub user: CurrentUser,
    pub city: City,
    pub request_id: String,
    pub city_access: CityAccess,
    pub access: Access,
}

impl Actor {
    pub fn user_id(&self) -> &UserId {
        &self.user.user_id
    }

    pub fn role(&self) -> Role {
        self.user.role
    }

    pub fn host_ids(&self) -> &[HostId] {
        &self.access.host_ids
    }

    pub fn require(&self, permission: &str) -> Result<()> {
        self.access.require(permission)
    }

    pub fn require_scoped(
        &self,
        permission: &str,
        host_id: Option<&str>,
        property_id: Option<&str>,
    ) -> Result<()> {
        self.access.require_scoped(permission, host_id, property_id)
    }

    pub fn require_scoped_any(
        &self,
        permissions: &[&str],
        host_id: Option<&str>,
        property_id: Option<&str>,
    ) -> Result<()> {
        self.access.require_scoped_any(permissions, host_id, property_id)
    }

    pub fn require_platform(&self, permission: &str) -> Result<()> {
        self.access.require_platform(permission)
    }

    pub fn ensure_host_scope(&self, permission: &str, host_id: &str, property_id: &str) -> Result<()> {
        self.access.ensure_host_scope(permission, host_id, property_id)
    }

    pub fn allows(&self, permission: &str, host_id: &str, property_id: &str) -> bool {
        self.access.allows(permission, host_id, property_id)
    }

    pub fn scoped_access(&self, permission: &str) -> (Vec<HostId>, Vec<String>) {
        self.access.scoped_access(permission)
    }

    pub fn ensure_city(&self, resource: City) -> Result<()> {
        crate::city::ensure_resource_in_route_city(self.city, resource)
    }

    pub fn ensure_writable(&self) -> Result<()> {
        match self.city_access {
            CityAccess::Full => Ok(()),
            CityAccess::ViewOnly => Err(Error::forbidden(CITY_VIEW_ONLY)),
        }
    }

    pub fn audit(&self) -> AuditActor {
        AuditActor {
            user_id: self.user.user_id.clone(),
            city: self.city,
            permissions: self
                .access
                .grants
                .iter()
                .map(|grant| grant.permission.clone())
                .collect(),
            request_id: self.request_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn other_city_than_the_home_city_is_view_only() {
        assert_eq!(city_access_for(Some(City::KIN), Some(City::GOMA), None), CityAccess::ViewOnly);
        assert_eq!(city_access_for(Some(City::GOMA), Some(City::GOMA), None), CityAccess::Full);
    }

    #[test]
    fn token_mode_applies_without_an_explicit_city_pair() {
        assert_eq!(city_access_for(None, None, Some("view_only")), CityAccess::ViewOnly);
        assert_eq!(city_access_for(None, Some(City::GOMA), None), CityAccess::Full);
        assert_eq!(city_access_for(Some(City::GOMA), None, Some("full")), CityAccess::Full);
    }
}
