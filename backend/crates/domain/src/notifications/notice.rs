use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    User(String),
    PlatformTeam,
}

#[derive(Debug, Clone)]
pub struct Notice {
    pub event: &'static str,
    pub target: Target,
    pub params: Map<String, Value>,
    pub entity: Option<(String, String)>,
    pub actor_id: Option<String>,
    pub dedupe_key: Option<String>,
}

impl Notice {
    pub fn user(event: &'static str, user_id: impl Into<String>) -> Self {
        Self::new(event, Target::User(user_id.into()))
    }

    pub fn platform_team(event: &'static str) -> Self {
        Self::new(event, Target::PlatformTeam)
    }

    fn new(event: &'static str, target: Target) -> Self {
        Self { event, target, params: Map::new(), entity: None, actor_id: None, dedupe_key: None }
    }

    pub fn param(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.params.insert(key.to_string(), value.into());
        self
    }

    pub fn entity(mut self, entity_type: &str, entity_id: impl Into<String>) -> Self {
        self.entity = Some((entity_type.to_string(), entity_id.into()));
        self
    }

    pub fn actor(mut self, actor_id: Option<&str>) -> Self {
        self.actor_id = actor_id.filter(|value| !value.is_empty()).map(ToString::to_string);
        self
    }

    pub fn dedupe(mut self, key: impl Into<String>) -> Self {
        self.dedupe_key = Some(key.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_collects_params_entity_and_dedupe() {
        let notice = Notice::user("stay.message.host", "u1")
            .param("preview", "hi")
            .entity("reservation", "r1")
            .dedupe("k")
            .actor(Some(""));
        assert_eq!(notice.target, Target::User("u1".into()));
        assert_eq!(notice.params["preview"], "hi");
        assert_eq!(notice.entity, Some(("reservation".into(), "r1".into())));
        assert_eq!(notice.dedupe_key.as_deref(), Some("k"));
        assert!(notice.actor_id.is_none());
    }
}
