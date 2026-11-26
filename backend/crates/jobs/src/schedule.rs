use std::future::Future;
use std::time::Duration;

use kernel::Result;

use crate::stop::{is_stopped, pause, Stop};

pub async fn every<C, F, Fut>(
    name: &'static str,
    interval: Duration,
    batch: usize,
    context: C,
    mut stop: Stop,
    job: F,
) where
    C: Clone,
    F: Fn(C) -> Fut,
    Fut: Future<Output = Result<usize>>,
{
    loop {
        if is_stopped(&stop) {
            return;
        }
        let wait = match job(context.clone()).await {
            Ok(done) if done >= batch => Duration::ZERO,
            Ok(_) => interval,
            Err(error) => {
                tracing::error!(job = name, error = %error, "scheduled job failed");
                interval
            }
        };
        if pause(&mut stop, wait).await {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::sync::watch;

    #[tokio::test]
    async fn runs_repeatedly_until_stopped() {
        let (sender, stop) = watch::channel(false);
        let runs = Arc::new(AtomicUsize::new(0));
        let counter = runs.clone();
        let handle = tokio::spawn(every("test", Duration::from_millis(5), 10, counter, stop, |counter| async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok(0)
        }));
        tokio::time::sleep(Duration::from_millis(60)).await;
        sender.send(true).unwrap();
        handle.await.unwrap();
        assert!(runs.load(Ordering::SeqCst) >= 2);
    }

    #[tokio::test]
    async fn a_full_batch_runs_again_without_waiting() {
        let (sender, stop) = watch::channel(false);
        let runs = Arc::new(AtomicUsize::new(0));
        let counter = runs.clone();
        let handle = tokio::spawn(every("test", Duration::from_secs(3600), 5, counter, stop, |counter| async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok(5)
        }));
        tokio::time::sleep(Duration::from_millis(50)).await;
        sender.send(true).unwrap();
        handle.await.unwrap();
        assert!(runs.load(Ordering::SeqCst) > 3);
    }

    #[tokio::test]
    async fn failures_do_not_end_the_loop() {
        let (sender, stop) = watch::channel(false);
        let runs = Arc::new(AtomicUsize::new(0));
        let counter = runs.clone();
        let handle = tokio::spawn(every("test", Duration::from_millis(5), 10, counter, stop, |counter| async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(kernel::Error::internal("boom"))
        }));
        tokio::time::sleep(Duration::from_millis(60)).await;
        sender.send(true).unwrap();
        handle.await.unwrap();
        assert!(runs.load(Ordering::SeqCst) >= 2);
    }
}
