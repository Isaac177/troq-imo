use futures::future::BoxFuture;
use kernel::Result;

#[derive(Debug, Clone, Default)]
pub struct Contact {
    pub user_id: String,
    pub email: String,
    pub phone: String,
    pub language: String,
}

pub trait ContactDirectory: Send + Sync {
    fn contacts<'a>(&'a self, user_ids: &'a [String]) -> BoxFuture<'a, Result<Vec<Contact>>>;
}
