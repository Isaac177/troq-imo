use crate::delivery::TransactionalSubscriber;

#[derive(Default)]
pub struct Registry {
    subscribers: Vec<Box<dyn TransactionalSubscriber>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, subscriber: impl TransactionalSubscriber + 'static) {
        self.subscribers.push(Box::new(subscriber));
    }

    pub fn len(&self) -> usize {
        self.subscribers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscribers.is_empty()
    }

    pub fn as_refs(&self) -> Vec<&dyn TransactionalSubscriber> {
        self.subscribers.iter().map(|subscriber| subscriber.as_ref()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use async_trait::async_trait;
    use kernel::Result;
    use sqlx::{Postgres, Transaction};

    struct Named(&'static str);

    #[async_trait]
    impl TransactionalSubscriber for Named {
        fn name(&self) -> &'static str {
            self.0
        }

        fn accepts(&self, _event_type: &str) -> bool {
            true
        }

        async fn handle(&self, _tx: &mut Transaction<'_, Postgres>, _event: &Event) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn registry_keeps_registration_order() {
        let mut registry = Registry::new();
        assert!(registry.is_empty());
        registry.add(Named("notifications"));
        registry.add(Named("billing"));
        let names: Vec<_> = registry.as_refs().iter().map(|subscriber| subscriber.name()).collect();
        assert_eq!(names, vec!["notifications", "billing"]);
        assert_eq!(registry.len(), 2);
    }
}
