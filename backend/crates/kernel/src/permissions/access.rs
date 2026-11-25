use crate::error::codes::NOT_FOUND;
use crate::error::{Error, Result};
use crate::ids::HostId;

use super::grants::{Grant, Scope};

#[derive(Debug, Clone, Default)]
pub struct Access {
    pub grants: Vec<Grant>,
    pub host_ids: Vec<HostId>,
    pub auth_city: Option<String>,
}

impl Access {
    pub fn require(&self, permission: &str) -> Result<()> {
        if self.grants.iter().any(|grant| grant.permission == permission) {
            Ok(())
        } else {
            Err(Error::permission_missing(permission))
        }
    }

    pub fn require_scoped(
        &self,
        permission: &str,
        host_id: Option<&str>,
        property_id: Option<&str>,
    ) -> Result<()> {
        self.require_scoped_any(&[permission], host_id, property_id)
            .map_err(|_| Error::permission_missing(permission))
    }

    pub fn require_scoped_any(
        &self,
        permissions: &[&str],
        host_id: Option<&str>,
        property_id: Option<&str>,
    ) -> Result<()> {
        let allowed = self.grants.iter().any(|grant| {
            permissions.contains(&grant.permission.as_str()) && grant.covers(host_id, property_id)
        });
        if allowed {
            Ok(())
        } else {
            Err(Error::forbidden(crate::error::codes::PERMISSION_MISSING))
        }
    }

    pub fn require_platform(&self, permission: &str) -> Result<()> {
        let allowed = self
            .grants
            .iter()
            .any(|grant| grant.permission == permission && grant.scope == Scope::Platform);
        if allowed {
            Ok(())
        } else {
            Err(Error::permission_missing(permission))
        }
    }

    pub fn is_member_of(&self, host_id: &str) -> bool {
        self.host_ids.iter().any(|id| id.as_str() == host_id)
    }

    pub fn ensure_host_scope(
        &self,
        permission: &str,
        host_id: &str,
        property_id: &str,
    ) -> Result<()> {
        if !self.is_member_of(host_id) {
            return Err(Error::not_found(NOT_FOUND));
        }
        self.require_scoped(permission, Some(host_id), Some(property_id))
    }

    pub fn allows(&self, permission: &str, host_id: &str, property_id: &str) -> bool {
        self.ensure_host_scope(permission, host_id, property_id).is_ok()
    }

    pub fn scoped_access(&self, permission: &str) -> (Vec<HostId>, Vec<String>) {
        let matching = || self.grants.iter().filter(|grant| grant.permission == permission);
        let platform = matching().any(|grant| grant.scope == Scope::Platform);
        let full = self
            .host_ids
            .iter()
            .filter(|host| {
                platform
                    || matching().any(|grant| matches!(&grant.scope, Scope::Host(scope) if scope == *host))
            })
            .cloned()
            .collect();
        let properties = matching()
            .filter_map(|grant| match &grant.scope {
                Scope::Property(id) => Some(id.as_str().to_string()),
                _ => None,
            })
            .collect();
        (full, properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};

    fn access(grants: Vec<Grant>, hosts: &[&str]) -> Access {
        Access {
            grants,
            host_ids: hosts.iter().map(|id| HostId::new(*id)).collect(),
            auth_city: None,
        }
    }

    #[test]
    fn require_accepts_any_scope_and_rejects_missing_permission() {
        let access = access(vec![Grant::host(PROPERTIES_READ, "h1")], &["h1"]);
        assert!(access.require(PROPERTIES_READ).is_ok());
        assert_eq!(access.require(PROPERTIES_WRITE).unwrap_err().code().as_str(), "PERMISSION_MISSING");
    }

    #[test]
    fn non_member_host_is_not_found_before_permissions_are_checked() {
        let access = access(vec![Grant::platform(PROPERTIES_READ)], &["h1"]);
        let error = access.ensure_host_scope(PROPERTIES_READ, "h2", "p1").unwrap_err();
        assert_eq!(error.code().as_str(), "NOT_FOUND");
        assert!(access.ensure_host_scope(PROPERTIES_READ, "h1", "p1").is_ok());
    }

    #[test]
    fn scoped_access_splits_full_hosts_and_single_properties() {
        let access = access(
            vec![Grant::host(PROPERTIES_READ, "h1"), Grant::property(PROPERTIES_READ, "p9")],
            &["h1", "h2"],
        );
        let (hosts, properties) = access.scoped_access(PROPERTIES_READ);
        assert_eq!(hosts, vec![HostId::new("h1")]);
        assert_eq!(properties, vec!["p9".to_string()]);
    }

    #[test]
    fn platform_grant_gives_full_access_to_every_member_host() {
        let access = access(vec![Grant::platform(PROPERTIES_READ)], &["h1", "h2"]);
        assert_eq!(access.scoped_access(PROPERTIES_READ).0.len(), 2);
    }

    #[test]
    fn require_platform_ignores_host_scoped_grants() {
        let access = access(vec![Grant::host(PROPERTIES_READ, "h1")], &["h1"]);
        assert!(access.require_platform(PROPERTIES_READ).is_err());
    }

    #[test]
    fn require_scoped_any_accepts_one_of_several() {
        let access = access(vec![Grant::property(PROPERTIES_WRITE, "p1")], &["h1"]);
        assert!(access.require_scoped_any(&[PROPERTIES_READ, PROPERTIES_WRITE], Some("h1"), Some("p1")).is_ok());
        assert!(access.require_scoped_any(&[PROPERTIES_READ], Some("h1"), Some("p1")).is_err());
    }
}
