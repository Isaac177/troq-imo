use kernel::Result;
use sqlx::PgPool;

pub async fn balance_minor(pool: &PgPool, host_id: &str) -> Result<i64> {
    let balance: Option<i64> =
        sqlx::query_scalar(r#"SELECT (balance * 100)::bigint FROM stay_credit_accounts WHERE "operatorId" = $1"#)
            .bind(host_id)
            .fetch_optional(pool)
            .await?;
    Ok(balance.unwrap_or(0))
}
