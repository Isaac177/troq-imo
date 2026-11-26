use futures::future::BoxFuture;
use kernel::Result;
use sqlx::PgPool;

use crate::notifications::{Contact, ContactDirectory};

#[derive(Clone)]
pub struct IdentityContacts {
    pool: PgPool,
}

impl IdentityContacts {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ContactDirectory for IdentityContacts {
    fn contacts<'a>(&'a self, user_ids: &'a [String]) -> BoxFuture<'a, Result<Vec<Contact>>> {
        Box::pin(async move {
            if user_ids.is_empty() {
                return Ok(Vec::new());
            }
            let rows = sqlx::query_as::<_, (String, Option<String>, Option<String>, String)>(
                "SELECT id, email, phone, preferred_language FROM identity_users WHERE id = ANY($1) AND status = 'ACTIVE' ORDER BY id",
            )
            .bind(user_ids)
            .fetch_all(&self.pool)
            .await?;
            Ok(rows.into_iter().map(|(user_id, email, phone, language)| Contact {
                user_id,
                email: email.unwrap_or_default(),
                phone: phone.unwrap_or_default(),
                language,
            }).collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn returns_only_active_requested_users_with_contacts(pool: PgPool) {
        sqlx::query(
            "INSERT INTO identity_users (id, home_city_id, status, email, phone, preferred_language) VALUES ('seed-contact-active', 'GOMA', 'ACTIVE', 'guest@example.com', NULL, 'fr'), ('seed-contact-suspended', 'KIN', 'SUSPENDED', 'suspended@example.com', '+243800000010', 'en'), ('seed-contact-unrequested', 'GOMA', 'ACTIVE', 'other@example.com', NULL, 'en')",
        ).execute(&pool).await.unwrap();
        let directory = IdentityContacts::new(pool);
        let users = vec!["seed-contact-active".to_owned(), "seed-contact-suspended".to_owned(), "missing".to_owned()];
        let contacts = directory.contacts(&users).await.unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].user_id, "seed-contact-active");
        assert_eq!(contacts[0].email, "guest@example.com");
        assert_eq!(contacts[0].phone, "");
        assert_eq!(contacts[0].language, "fr");
        assert!(directory.contacts(&[]).await.unwrap().is_empty());
    }
}
