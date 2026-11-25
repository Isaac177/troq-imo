use std::sync::Arc;

use async_trait::async_trait;
use futures::future::BoxFuture;
use kernel::{Kernel, Result};

use crate::event::Event;

#[async_trait]
pub trait Subscriber: Send + Sync {
    fn name(&self) -> &'static str;

    fn event_types(&self) -> &'static [&'static str];

    async fn handle(&self, kernel: &Kernel, event: &Event) -> Result<()>;
}

pub fn matches(pattern: &str, event_type: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    match pattern.strip_suffix(".*") {
        Some(prefix) => event_type
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('.')),
        None => pattern == event_type,
    }
}

type Handler =
    Arc<dyn for<'a> Fn(&'a Kernel, &'a Event) -> BoxFuture<'a, Result<()>> + Send + Sync>;

pub struct FnSubscriber {
    name: &'static str,
    event_types: &'static [&'static str],
    handler: Handler,
}

impl FnSubscriber {
    pub fn new<F>(name: &'static str, event_types: &'static [&'static str], handler: F) -> Self
    where
        F: for<'a> Fn(&'a Kernel, &'a Event) -> BoxFuture<'a, Result<()>> + Send + Sync + 'static,
    {
        Self {
            name,
            event_types,
            handler: Arc::new(handler),
        }
    }
}

#[async_trait]
impl Subscriber for FnSubscriber {
    fn name(&self) -> &'static str {
        self.name
    }

    fn event_types(&self) -> &'static [&'static str] {
        self.event_types
    }

    async fn handle(&self, kernel: &Kernel, event: &Event) -> Result<()> {
        (self.handler)(kernel, event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_pattern_matches_only_that_type() {
        assert!(matches("reservation.held", "reservation.held"));
        assert!(!matches("reservation.held", "reservation.held_late"));
    }

    #[test]
    fn prefix_pattern_matches_the_family() {
        assert!(matches("reservation.*", "reservation.expired"));
        assert!(matches("reservation.*", "reservation.amendment.accepted"));
        assert!(!matches("reservation.*", "reservationx.expired"));
        assert!(!matches("reservation.*", "reservation"));
        assert!(!matches("case.*", "reservation.case.closed"));
    }

    #[test]
    fn star_matches_everything() {
        assert!(matches("*", "property.created"));
    }
}
