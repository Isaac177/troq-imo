use kernel::auth::Actor;
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::{PgPool, Postgres, QueryBuilder};

use super::credit_types::{AccountFilter, AccountRecord, AccountRow, CreditDetail, CreditSummary};
use crate::billing::amounts::wire;
use crate::billing::credits::state::disabled_sql;
use crate::billing::messages::{CREDIT_STATUS_UNKNOWN, HOST_NOT_FOUND};
use crate::billing::{scope, settings};

const STATUSES: [&str; 3] = ["ENABLED", "DISABLED", "LOW"];

const ROW_COLUMNS: &str = "x.operator_id, x.operator_name, x.rate_bps, x.balance_minor, NOT x.disabled AS enabled, x.low, x.last_top_up_at";

const TOTAL_COLUMNS: &str = "COUNT(*), COUNT(*) FILTER (WHERE x.disabled), COUNT(*) FILTER (WHERE x.low), COALESCE(SUM(x.balance_minor), 0)::bigint";

fn escape_like(value: &str) -> String {
    value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

fn base_head() -> String {
    format!(
        r#"WITH base AS (
            SELECT h.id AS operator_id, h.display_name AS operator_name,
                   COALESCE((SELECT cr."rateBps" FROM stay_operator_commission_rates cr
                              WHERE cr."operatorId" = h.id AND cr."effectiveFrom" <= now()
                              ORDER BY cr."effectiveFrom" DESC, cr."createdAt" DESC, cr.id DESC LIMIT 1),
                             (SELECT ps."valueInt" FROM stay_platform_settings ps
                               WHERE ps.city_id = h.city_id AND ps.key = 'DEFAULT_COMMISSION_BPS'), 0) AS rate_bps,
                   (COALESCE(a.balance, 0) * 100)::bigint AS balance_minor,
                   {disabled} AS disabled,
                   (SELECT MAX(e."createdAt") FROM stay_credit_entries e
                     WHERE e."operatorId" = h.id AND e."entryType" = 'TOPUP') AS last_top_up_at
              FROM hosts h
              LEFT JOIN stay_credit_accounts a ON a."operatorId" = h.id
             WHERE h.city_id = "#,
        disabled = disabled_sql("h.id", "h.city_id")
    )
}

fn builder<'a>(select: &str, filter: &'a AccountFilter, city: &'a str, low_minor: i64) -> QueryBuilder<'a, Postgres> {
    let mut builder = QueryBuilder::<Postgres>::new(base_head());
    builder.push_bind(city);
    builder.push(") SELECT ").push(select);
    builder.push(" FROM (SELECT b.*, (NOT b.disabled AND b.rate_bps > 0 AND b.balance_minor < ");
    builder.push_bind(low_minor).push(") AS low FROM base b) x WHERE TRUE");
    if let Some(host_id) = filter.host_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        builder.push(" AND x.operator_id = ").push_bind(host_id);
    }
    match filter.status.as_deref() {
        Some("ENABLED") => {
            builder.push(" AND NOT x.disabled");
        }
        Some("DISABLED") => {
            builder.push(" AND x.disabled");
        }
        Some("LOW") => {
            builder.push(" AND x.low");
        }
        _ => {}
    }
    if let Some(search) = filter.search.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        builder
            .push(" AND x.operator_name ILIKE ")
            .push_bind(format!("%{}%", escape_like(search)))
            .push(r" ESCAPE '\'");
    }
    builder
}

fn validate(filter: &AccountFilter) -> Result<()> {
    match filter.status.as_deref() {
        Some(status) if !STATUSES.contains(&status) => {
            Err(Error::bad_request(CREDIT_STATUS_UNKNOWN).with_params(json!({ "status": status })))
        }
        _ => Ok(()),
    }
}

async fn low_minor(pool: &PgPool, actor: &Actor) -> Result<i64> {
    Ok(i64::from(settings::low_credit_cdf(pool, actor.city).await?) * 100)
}

pub async fn list(pool: &PgPool, actor: &Actor, filter: AccountFilter, page: PageRequest) -> Result<Page<AccountRow>> {
    scope::require_staff(actor)?;
    validate(&filter)?;
    let page = page.validated()?;
    let low = low_minor(pool, actor).await?;
    let city = actor.city.code();
    let total: i64 = builder("COUNT(*)", &filter, city, low).build_query_scalar().fetch_one(pool).await?;
    let mut items_builder = builder(ROW_COLUMNS, &filter, city, low);
    items_builder
        .push(" ORDER BY x.disabled DESC, x.low DESC, x.operator_name, x.operator_id LIMIT ")
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let records = items_builder.build_query_as::<AccountRecord>().fetch_all(pool).await?;
    Ok(Page::new(records.into_iter().map(AccountRow::from).collect(), total, page))
}

pub async fn summary(pool: &PgPool, actor: &Actor, host_id: Option<String>) -> Result<CreditSummary> {
    scope::require_staff(actor)?;
    let filter = AccountFilter { host_id, ..AccountFilter::default() };
    let low = low_minor(pool, actor).await?;
    let city = actor.city.code();
    let (accounts, disabled, low_count, total_minor) = builder(TOTAL_COLUMNS, &filter, city, low)
        .build_query_as::<(i64, i64, i64, i64)>()
        .fetch_one(pool)
        .await?;
    let (topped_up, taken, refunded): (i64, i64, i64) = sqlx::query_as(
        r#"SELECT (COALESCE(SUM(e.amount) FILTER (WHERE e."entryType" = 'TOPUP'), 0) * 100)::bigint,
                  (COALESCE(-SUM(e.amount) FILTER (WHERE e."entryType" = 'COMMISSION'), 0) * 100)::bigint,
                  (COALESCE(SUM(e.amount) FILTER (WHERE e."entryType" = 'REFUND'), 0) * 100)::bigint
             FROM stay_credit_entries e
            WHERE e.city_id = $1 AND e."createdAt" >= now() - interval '30 days'
              AND ($2::text IS NULL OR e."operatorId" = $2)"#,
    )
    .bind(city)
    .bind(filter.host_id.as_deref())
    .fetch_one(pool)
    .await?;
    Ok(CreditSummary {
        accounts,
        disabled,
        low: low_count,
        total_balance: wire(total_minor),
        topped_up_30d: wire(topped_up),
        commission_taken_30d: wire(taken),
        refunded_30d: wire(refunded),
    })
}

pub async fn detail(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<CreditDetail> {
    scope::require_staff(actor)?;
    let low_threshold = settings::low_credit_cdf(pool, actor.city).await?;
    let filter = AccountFilter { host_id: Some(host_id.to_string()), ..AccountFilter::default() };
    let record = builder(ROW_COLUMNS, &filter, actor.city.code(), i64::from(low_threshold) * 100)
        .build_query_as::<AccountRecord>()
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(HOST_NOT_FOUND))?;
    Ok(CreditDetail {
        host_id: record.operator_id,
        rate_bps: record.rate_bps,
        balance: wire(record.balance_minor),
        enabled: record.enabled,
        low: record.low,
        low_threshold,
    })
}
