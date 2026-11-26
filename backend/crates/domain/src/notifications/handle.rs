use kernel::Result;
use serde_json::json;
use sqlx::{PgConnection, Postgres, Transaction};

use super::deliveries::{self, Channel, NewDelivery};
use super::dispatch::{send, EventContext};
use super::documents::DocumentKind;
use super::property_events::{case_notices, property_notices};
use super::reservation_events::notices;

const FAMILIES: [&str; 3] = ["reservation.", "property.", "case."];

pub fn accepts(event_type: &str) -> bool {
    FAMILIES.iter().any(|family| event_type.starts_with(family))
}

pub fn document_for(event_type: &str) -> Option<DocumentKind> {
    match event_type {
        "reservation.held" | "reservation.request_accepted" => Some(DocumentKind::StayVoucher),
        "reservation.cancelled" => Some(DocumentKind::StayCancellation),
        "reservation.damage_claimed" => Some(DocumentKind::StayDamageClaim),
        _ => None,
    }
}

async fn enqueue_document(conn: &mut PgConnection, ctx: &EventContext<'_>) -> Result<()> {
    let Some(kind) = document_for(ctx.event_type) else {
        return Ok(());
    };
    let key = format!("{}:{}:{}", ctx.event_id, Channel::Document.as_str(), kind.as_str());
    let delivery = NewDelivery {
        city: ctx.city,
        dedupe_key: &key,
        channel: Channel::Document,
        user_id: None,
        notification_id: None,
        payload: json!({ "document": kind.as_str(), "snapshot": ctx.data }),
    };
    deliveries::enqueue(conn, delivery).await
}

pub async fn handle_event(tx: &mut Transaction<'_, Postgres>, ctx: &EventContext<'_>) -> Result<()> {
    let found = match ctx.event_type.split('.').next().unwrap_or_default() {
        "reservation" => notices(ctx.event_type, ctx.data),
        "property" => property_notices(ctx.event_type, ctx.data),
        "case" => case_notices(ctx.event_type, ctx.data),
        _ => Vec::new(),
    };
    for notice in found {
        send(&mut **tx, ctx, notice).await?;
    }
    enqueue_document(&mut **tx, ctx).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_three_event_families() {
        assert!(accepts("reservation.held"));
        assert!(accepts("property.revision.submitted"));
        assert!(accepts("case.closed"));
        assert!(!accepts("operations.task.created"));
        assert!(!accepts("claim.decided"));
    }

    #[test]
    fn four_events_issue_documents() {
        assert_eq!(document_for("reservation.held"), Some(DocumentKind::StayVoucher));
        assert_eq!(document_for("reservation.request_accepted"), Some(DocumentKind::StayVoucher));
        assert_eq!(document_for("reservation.cancelled"), Some(DocumentKind::StayCancellation));
        assert_eq!(document_for("reservation.damage_claimed"), Some(DocumentKind::StayDamageClaim));
        assert_eq!(document_for("reservation.expired"), None);
    }
}
