use futures::future::BoxFuture;

use crate::error::Result;
use crate::ids::UserId;
use crate::permissions::Access;

pub trait AccessDirectory: Send + Sync {
    fn access_for<'a>(&'a self, user_id: &'a UserId) -> BoxFuture<'a, Result<Access>>;

    fn revoked_before<'a>(&'a self, user_id: &'a UserId) -> BoxFuture<'a, Result<Option<i64>>>;
}

pub fn is_revoked(revoked_before: Option<i64>, issued_at: i64) -> bool {
    revoked_before.is_some_and(|cutoff| issued_at < cutoff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_issued_before_the_cutoff_are_revoked() {
        assert!(is_revoked(Some(100), 99));
        assert!(!is_revoked(Some(100), 100));
        assert!(!is_revoked(None, 1));
    }
}
