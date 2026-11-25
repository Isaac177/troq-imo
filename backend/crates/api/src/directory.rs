use futures::future::BoxFuture;
use kernel::auth::AccessDirectory;
use kernel::error::{codes, Error, Result};
use kernel::ids::UserId;
use kernel::permissions::Access;

pub struct ClosedDirectory;

impl AccessDirectory for ClosedDirectory {
    fn access_for<'a>(&'a self, _user_id: &'a UserId) -> BoxFuture<'a, Result<Access>> {
        Box::pin(async { Err(Error::unauthorized(codes::AUTH_REQUIRED)) })
    }

    fn revoked_before<'a>(&'a self, _user_id: &'a UserId) -> BoxFuture<'a, Result<Option<i64>>> {
        Box::pin(async { Err(Error::unauthorized(codes::AUTH_REQUIRED)) })
    }
}
