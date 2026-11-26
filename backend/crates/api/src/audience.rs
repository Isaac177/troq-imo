#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audience {
    Public,
    Guest,
    Host,
    Admin,
}

impl Audience {
    pub const ALL: [Audience; 4] = [Self::Public, Self::Guest, Self::Host, Self::Admin];

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Public => "/api/public",
            Self::Guest => "/api/guest",
            Self::Host => "/api/host",
            Self::Admin => "/api/admin",
        }
    }

    pub fn requires_auth(self) -> bool {
        self != Self::Public
    }

    pub fn of_path(path: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|audience| {
            path.strip_prefix(audience.prefix())
                .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_paths_to_their_audience() {
        assert_eq!(Audience::of_path("/api/host/reservations"), Some(Audience::Host));
        assert_eq!(Audience::of_path("/api/public"), Some(Audience::Public));
        assert_eq!(Audience::of_path("/api/hostile/x"), None);
        assert_eq!(Audience::of_path("/health"), None);
    }

    #[test]
    fn only_public_skips_authentication() {
        assert!(!Audience::Public.requires_auth());
        assert!(Audience::Admin.requires_auth());
    }
}
