use async_trait::async_trait;
use domain::notifications::{self, EventContext};
use kernel::Result;
use sqlx::{Postgres, Transaction};

use crate::delivery::TransactionalSubscriber;
use crate::event::Event;

pub struct NotificationsSubscriber;

#[async_trait]
impl TransactionalSubscriber for NotificationsSubscriber {
    fn name(&self) -> &'static str {
        "notifications"
    }

    fn accepts(&self, event_type: &str) -> bool {
        notifications::accepts(event_type)
    }

    async fn handle(&self, tx: &mut Transaction<'_, Postgres>, event: &Event) -> Result<()> {
        let context = EventContext {
            event_id: &event.id,
            city: event.city,
            event_type: &event.event_type,
            data: &event.data,
        };
        notifications::handle_event(tx, &context).await
    }
}
