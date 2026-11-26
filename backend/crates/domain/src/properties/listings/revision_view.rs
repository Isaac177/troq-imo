use kernel::error::{Error, Result};
use sqlx::PgPool;

use super::queries;
use crate::properties::messages::REVISION_NOT_FOUND;
use crate::properties::revisions::content::ListingContent;
use crate::properties::revisions::types::{RevisionRow, RevisionView};

pub fn view(row: RevisionRow) -> Result<RevisionView> {
    let content = ListingContent::from_snapshot(&row.snapshot)?;
    Ok(RevisionView { summary: row.summary, content })
}

pub async fn load(pool: &PgPool, revision_id: &str) -> Result<RevisionView> {
    view(queries::find_revision(pool, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?)
}
