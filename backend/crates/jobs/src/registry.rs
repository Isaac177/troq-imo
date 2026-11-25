use std::sync::Arc;

use crate::subscriber::{matches, Subscriber};

#[derive(Default)]
pub struct Registry {
    subscribers: Vec<Arc<dyn Subscriber>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, subscriber: impl Subscriber + 'static) {
        self.subscribers.push(Arc::new(subscriber));
    }

    pub fn len(&self) -> usize {
        self.subscribers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscribers.is_empty()
    }

    pub fn matching<'a>(
        &'a self,
        event_type: &'a str,
    ) -> impl Iterator<Item = &'a Arc<dyn Subscriber>> + 'a {
        self.subscribers.iter().filter(move |subscriber| {
            subscriber
                .event_types()
                .iter()
                .any(|pattern| matches(pattern, event_type))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use async_trait::async_trait;
    use kernel::{Kernel, Result};

    struct Named(&'static str, &'static [&'static str]);

    #[async_trait]
    impl Subscriber for Named {
        fn name(&self) -> &'static str {
            self.0
        }

        fn event_types(&self) -> &'static [&'static str] {
            self.1
        }

        async fn handle(&self, _kernel: &Kernel, _event: &Event) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn matching_returns_only_interested_subscribers_in_order() {
        let mut registry = Registry::new();
        registry.add(Named("notifications", &["reservation.*", "case.closed"]));
        registry.add(Named("billing", &["reservation.checked_out"]));
        registry.add(Named("search", &["property.*"]));
        let names: Vec<_> = registry
            .matching("reservation.checked_out")
            .map(|subscriber| subscriber.name())
            .collect();
        assert_eq!(names, vec!["notifications", "billing"]);
        assert_eq!(registry.matching("case.opened").count(), 0);
        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn an_empty_registry_matches_nothing() {
        let registry = Registry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.matching("reservation.held").count(), 0);
    }
}
