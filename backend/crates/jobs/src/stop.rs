use std::time::Duration;

use tokio::sync::watch;

pub type Stop = watch::Receiver<bool>;

pub fn is_stopped(stop: &Stop) -> bool {
    *stop.borrow()
}

pub async fn pause(stop: &mut Stop, duration: Duration) -> bool {
    if is_stopped(stop) {
        return true;
    }
    tokio::select! {
        _ = tokio::time::sleep(duration) => is_stopped(stop),
        _ = stop.changed() => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn pause_ends_early_when_stop_is_signalled() {
        let (sender, mut receiver) = watch::channel(false);
        let waiter = tokio::spawn(async move { pause(&mut receiver, Duration::from_secs(60)).await });
        sender.send(true).unwrap();
        assert!(waiter.await.unwrap());
    }

    #[tokio::test]
    async fn pause_returns_false_after_a_quiet_interval() {
        let (_sender, mut receiver) = watch::channel(false);
        assert!(!pause(&mut receiver, Duration::from_millis(5)).await);
    }
}
