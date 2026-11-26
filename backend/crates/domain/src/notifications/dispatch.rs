use kernel::{City, Result};
use serde_json::{json, Value};
use sqlx::PgConnection;

use super::catalog;
use super::copy;
use super::deliveries::{self, Channel, NewDelivery};
use super::inbox_write::{self, NewNotification};
use super::links::deep_link;
use super::notice::Notice;
use super::prefs;
use super::recipients;
use super::render::fill;

pub struct EventContext<'a> {
    pub event_id: &'a str,
    pub aggregate_id: &'a str,
    pub city: City,
    pub event_type: &'a str,
    pub data: &'a Value,
}

pub async fn send(conn: &mut PgConnection, ctx: &EventContext<'_>, notice: Notice) -> Result<usize> {
    let (Some(def), Some(template)) = (catalog::find(notice.event), copy::find(notice.event)) else {
        tracing::warn!(event = notice.event, "notifications.unknown_event");
        return Ok(0);
    };
    let mut user_ids = recipients::resolve(conn, &notice.target).await?;
    if let Some(actor) = notice.actor_id.as_deref() {
        user_ids.retain(|id| id != actor);
    }
    if let (Some(key), false) = (notice.dedupe_key.as_deref(), user_ids.is_empty()) {
        let duplicates = inbox_write::recent_duplicates(conn, &user_ids, key).await?;
        user_ids.retain(|id| !duplicates.contains(id));
    }
    if user_ids.is_empty() {
        return Ok(0);
    }
    let channels = prefs::effective(conn, &user_ids, def).await?;
    let title_en = fill(template.title_en, &notice.params);
    let title_fr = fill(template.title_fr, &notice.params);
    let body_en = fill(template.body_en, &notice.params);
    let body_fr = fill(template.body_fr, &notice.params);
    let (entity_type, entity_id) = match &notice.entity {
        Some((kind, id)) => (Some(kind.as_str()), Some(id.as_str())),
        None => (None, None),
    };
    let link = deep_link(def.audience, entity_type, entity_id);
    let mut delivered = 0;
    for user_id in &user_ids {
        let chosen = channels.get(user_id).copied().unwrap_or_else(|| prefs::defaults(def));
        if !chosen.in_app && !chosen.push && !chosen.email {
            continue;
        }
        let input = NewNotification {
            city: ctx.city,
            user_id,
            db_type: def.db_type,
            priority: def.priority,
            title_en: &title_en,
            title_fr: &title_fr,
            body_en: &body_en,
            body_fr: &body_fr,
            deep_link: &link,
            event_key: def.key,
            category: def.category,
            audience: def.audience,
            params: &notice.params,
            entity_type,
            entity_id,
            actor_id: notice.actor_id.as_deref(),
            dedupe_key: notice.dedupe_key.as_deref(),
            source_event_id: ctx.event_id,
        };
        let Some(notification_id) = inbox_write::insert(conn, &input).await? else {
            continue;
        };
        delivered += 1;
        let payload = json!({ "titleEn": title_en, "titleFr": title_fr, "bodyEn": body_en, "bodyFr": body_fr, "audience": def.audience, "deepLink": link });
        for (wanted, channel) in [(chosen.push, Channel::Push), (chosen.email, Channel::Email)] {
            if wanted {
                let key = format!("{}:{}:{user_id}:{}", ctx.event_id, channel.as_str(), def.key);
                let delivery = NewDelivery {
                    city: ctx.city,
                    dedupe_key: &key,
                    channel,
                    user_id: Some(user_id),
                    notification_id: Some(&notification_id),
                    payload: payload.clone(),
                };
                deliveries::enqueue(conn, delivery).await?;
            }
        }
    }
    Ok(delivered)
}
