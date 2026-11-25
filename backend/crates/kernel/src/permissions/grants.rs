use serde::{Deserialize, Serialize};

use crate::ids::{HostId, PropertyId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Platform,
    Host(HostId),
    Property(PropertyId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grant {
    pub permission: String,
    pub scope: Scope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantRow {
    pub permission: String,
    pub scope_type: String,
    #[serde(default)]
    pub scope_id: Option<String>,
}

impl Grant {
    pub fn platform(permission: &str) -> Self {
        Self {
            permission: permission.to_string(),
            scope: Scope::Platform,
        }
    }

    pub fn host(permission: &str, host_id: &str) -> Self {
        Self {
            permission: permission.to_string(),
            scope: Scope::Host(HostId::new(host_id)),
        }
    }

    pub fn property(permission: &str, property_id: &str) -> Self {
        Self {
            permission: permission.to_string(),
            scope: Scope::Property(PropertyId::new(property_id)),
        }
    }

    pub fn from_row(row: GrantRow) -> Option<Self> {
        let scope = match (row.scope_type.as_str(), row.scope_id) {
            ("PLATFORM", _) => Scope::Platform,
            ("HOST", Some(id)) => Scope::Host(HostId::new(id)),
            ("PROPERTY", Some(id)) => Scope::Property(PropertyId::new(id)),
            _ => return None,
        };
        Some(Self {
            permission: row.permission,
            scope,
        })
    }

    pub fn covers(&self, host_id: Option<&str>, property_id: Option<&str>) -> bool {
        match &self.scope {
            Scope::Platform => true,
            Scope::Host(scope) => host_id == Some(scope.as_str()),
            Scope::Property(scope) => property_id == Some(scope.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions::names::PROPERTIES_READ;

    #[test]
    fn host_grant_covers_only_its_own_host() {
        let grant = Grant::host(PROPERTIES_READ, "host-a");
        assert!(grant.covers(Some("host-a"), None));
        assert!(!grant.covers(Some("host-b"), None));
        assert!(!grant.covers(None, None));
    }

    #[test]
    fn property_grant_covers_only_its_own_property() {
        let grant = Grant::property(PROPERTIES_READ, "prop-a");
        assert!(grant.covers(None, Some("prop-a")));
        assert!(!grant.covers(Some("host-a"), Some("prop-b")));
    }

    #[test]
    fn platform_grant_covers_everything() {
        assert!(Grant::platform(PROPERTIES_READ).covers(None, None));
    }

    #[test]
    fn retired_or_incomplete_scopes_grant_nothing() {
        let retired = GrantRow {
            permission: PROPERTIES_READ.into(),
            scope_type: "OPERATOR".into(),
            scope_id: Some("host-a".into()),
        };
        assert!(Grant::from_row(retired).is_none());
        let missing_id = GrantRow {
            permission: PROPERTIES_READ.into(),
            scope_type: "HOST".into(),
            scope_id: None,
        };
        assert!(Grant::from_row(missing_id).is_none());
    }
}
