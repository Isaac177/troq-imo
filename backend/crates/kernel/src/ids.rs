use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn new_id(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::new_v4().simple())
}

pub fn new_event_id() -> String {
    Uuid::new_v4().to_string()
}

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

string_id!(UserId);
string_id!(HostId);
string_id!(PropertyId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_carry_their_prefix_and_are_unique() {
        let first = new_id("block");
        assert!(first.starts_with("block-"));
        assert_ne!(first, new_id("block"));
    }

    #[test]
    fn typed_ids_serialise_as_plain_strings() {
        let id = HostId::new("seed-host-1");
        assert_eq!(serde_json::to_string(&id).unwrap(), "\"seed-host-1\"");
        assert_eq!(id.as_str(), "seed-host-1");
    }
}
