use std::collections::HashMap;

use kernel::{Error, Result};
use sqlx::{PgConnection, PgPool};

use super::catalog::{self, EventDef};
use super::codes::PREFERENCE_KEY_INVALID;
use super::types::PreferenceView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channels {
    pub in_app: bool,
    pub push: bool,
    pub email: bool,
}

pub fn defaults(def: &EventDef) -> Channels {
    Channels { in_app: true, push: true, email: def.default_email }
}

fn precedence(key: &str, event_key: &str) -> u8 {
    if key == event_key {
        3
    } else if key.starts_with("category:") {
        2
    } else {
        1
    }
}

pub fn resolve(rows: &[(String, String, Channels)], user_ids: &[String], def: &EventDef) -> HashMap<String, Channels> {
    let mut best: HashMap<&str, (u8, Channels)> = HashMap::new();
    for (user_id, event_key, channels) in rows {
        let rank = precedence(event_key, def.key);
        let replace = best.get(user_id.as_str()).map(|(current, _)| rank > *current).unwrap_or(true);
        if replace {
            best.insert(user_id, (rank, *channels));
        }
    }
    let critical = def.priority == "URGENT";
    user_ids
        .iter()
        .map(|user_id| {
            let mut channels = best.get(user_id.as_str()).map(|(_, value)| *value).unwrap_or_else(|| defaults(def));
            if critical {
                channels.in_app = true;
                channels.email = true;
            }
            (user_id.clone(), channels)
        })
        .collect()
}

pub async fn effective(conn: &mut PgConnection, user_ids: &[String], def: &EventDef) -> Result<HashMap<String, Channels>> {
    let keys = vec![def.key.to_string(), format!("category:{}", def.category), "*".to_string()];
    let rows = sqlx::query_as::<_, (String, String, bool, bool, bool)>(
        "SELECT user_id, event_key, in_app, push, email FROM notification_event_preferences WHERE user_id = ANY($1) AND event_key = ANY($2)",
    )
    .bind(user_ids)
    .bind(&keys)
    .fetch_all(conn)
    .await?;
    let rows: Vec<_> = rows
        .into_iter()
        .map(|(user, key, in_app, push, email)| (user, key, Channels { in_app, push, email }))
        .collect();
    Ok(resolve(&rows, user_ids, def))
}

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<PreferenceView>> {
    let rows = sqlx::query_as::<_, (String, bool, bool, bool)>(
        "SELECT event_key, in_app, push, email FROM notification_event_preferences WHERE user_id = $1 ORDER BY event_key",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(event_key, in_app, push, email)| PreferenceView { event_key, in_app, push, email })
        .collect())
}

pub fn valid_key(key: &str) -> bool {
    key == "*"
        || catalog::find(key).is_some()
        || key.strip_prefix("category:").is_some_and(|category| category == "stays")
}

pub async fn upsert(pool: &PgPool, user_id: &str, event_key: &str, channels: Channels) -> Result<()> {
    if !valid_key(event_key) {
        return Err(Error::bad_request(PREFERENCE_KEY_INVALID));
    }
    sqlx::query(
        "INSERT INTO notification_event_preferences (user_id, event_key, in_app, push, email, updated_at) VALUES ($1, $2, $3, $4, $5, now()) ON CONFLICT (user_id, event_key) DO UPDATE SET in_app = EXCLUDED.in_app, push = EXCLUDED.push, email = EXCLUDED.email, updated_at = now()",
    )
    .bind(user_id)
    .bind(event_key)
    .bind(channels.in_app)
    .bind(channels.push)
    .bind(channels.email)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_off() -> Channels {
        Channels { in_app: false, push: false, email: false }
    }

    #[test]
    fn missing_rows_fall_back_to_the_event_defaults() {
        let def = catalog::find(catalog::RESERVATION_ACCEPTED_GUEST).unwrap();
        let resolved = resolve(&[], &["u1".to_string()], def);
        assert_eq!(resolved["u1"], Channels { in_app: true, push: true, email: true });
    }

    #[test]
    fn event_rule_beats_category_beats_wildcard() {
        let def = catalog::find(catalog::MESSAGE_HOST).unwrap();
        let rows = vec![
            ("u1".to_string(), "*".to_string(), all_off()),
            ("u1".to_string(), "category:stays".to_string(), Channels { in_app: true, push: false, email: false }),
            ("u1".to_string(), def.key.to_string(), Channels { in_app: true, push: true, email: true }),
        ];
        assert_eq!(resolve(&rows, &["u1".to_string()], def)["u1"], Channels { in_app: true, push: true, email: true });
        assert_eq!(resolve(&rows[..2], &["u1".to_string()], def)["u1"].push, false);
    }

    #[test]
    fn urgent_events_always_keep_in_app_and_email() {
        let def = catalog::find(catalog::PROPERTY_SUSPENDED_HOST).unwrap();
        let rows = vec![("u1".to_string(), "*".to_string(), all_off())];
        let channels = resolve(&rows, &["u1".to_string()], def)["u1"];
        assert!(channels.in_app && channels.email && !channels.push);
    }

    #[test]
    fn only_known_keys_are_accepted() {
        assert!(valid_key("*"));
        assert!(valid_key("category:stays"));
        assert!(valid_key("stay.message.host"));
        assert!(!valid_key("category:orders"));
        assert!(!valid_key("order.created.merchant"));
    }
}
