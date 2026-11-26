use kernel::error::Result;
use kernel::Kernel;

pub const SEARCH_EVENT_RETENTION_MONTHS: i32 = 6;

pub async fn purge_search_events(kernel: &Kernel, batch: i64) -> Result<usize> {
    let result = sqlx::query(
        r#"DELETE FROM stay_search_events
           WHERE id IN (
               SELECT id FROM stay_search_events
               WHERE "createdAt" < now() - make_interval(months => $1)
               ORDER BY "createdAt"
               LIMIT $2
           )"#,
    )
    .bind(SEARCH_EVENT_RETENTION_MONTHS)
    .bind(batch)
    .execute(kernel.db.pool())
    .await?;
    Ok(result.rows_affected() as usize)
}
