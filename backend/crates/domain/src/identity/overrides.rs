use kernel::permissions::Grant;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverrideEffect {
    Allow,
    Deny,
}

impl OverrideEffect {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "ALLOW",
            Self::Deny => "DENY",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PermissionOverride {
    pub grant: Grant,
    pub effect: OverrideEffect,
}

pub fn apply_overrides(grants: &mut Vec<Grant>, overrides: &[PermissionOverride]) {
    for item in overrides.iter().filter(|item| item.effect == OverrideEffect::Deny) {
        grants.retain(|grant| grant != &item.grant);
    }
    for item in overrides.iter().filter(|item| item.effect == OverrideEffect::Allow) {
        if !grants.contains(&item.grant) {
            grants.push(item.grant.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};

    fn change(grant: Grant, effect: OverrideEffect) -> PermissionOverride {
        PermissionOverride { grant, effect }
    }

    #[test]
    fn deny_removes_only_the_exact_permission_and_scope() {
        let denied = Grant::host(PROPERTIES_READ, "host-a");
        let other_scope = Grant::host(PROPERTIES_READ, "host-b");
        let other_permission = Grant::host(PROPERTIES_WRITE, "host-a");
        let mut grants = vec![denied.clone(), other_scope.clone(), other_permission.clone()];
        apply_overrides(&mut grants, &[change(denied, OverrideEffect::Deny)]);
        assert_eq!(grants, vec![other_scope, other_permission]);
    }

    #[test]
    fn allow_adds_the_grant_once_after_denials() {
        let grant = Grant::property(PROPERTIES_READ, "property-a");
        let mut grants = vec![grant.clone()];
        apply_overrides(&mut grants, &[
            change(grant.clone(), OverrideEffect::Deny),
            change(grant.clone(), OverrideEffect::Allow),
            change(grant.clone(), OverrideEffect::Allow),
        ]);
        assert_eq!(grants, vec![grant]);
    }

    #[test]
    fn denying_a_host_grant_does_not_remove_a_platform_grant() {
        let platform = Grant::platform(PROPERTIES_READ);
        let mut grants = vec![platform.clone()];
        apply_overrides(&mut grants, &[change(Grant::host(PROPERTIES_READ, "host-a"), OverrideEffect::Deny)]);
        assert_eq!(grants, vec![platform]);
    }
}
