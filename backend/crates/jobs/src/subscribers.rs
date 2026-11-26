use crate::notifications::NotificationsSubscriber;
use crate::registry::Registry;

pub fn register(registry: &mut Registry) {
    registry.add(NotificationsSubscriber);
}
