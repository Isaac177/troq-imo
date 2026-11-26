use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{City, Kernel, Page, Result};
use sqlx::{Postgres, QueryBuilder};

use super::reasons::ensure_status;
use super::types::{ClaimFilter, ClaimRow, ClaimSummary};
use crate::reservations::support::{escape_like, paging};

const FROM_SQL: &str = r#" FROM stay_cases c
   JOIN hosts o ON o.id = c."operatorId"
   LEFT JOIN stay_reservations r ON r.id = c."reservationId"
   LEFT JOIN stay_properties p ON p.id = r."propertyId"
   LEFT JOIN stay_claim_decisions d ON d."caseId" = c.id
   WHERE c."caseType" = 'DAMAGE_CLAIM'"#;

const SELECT_SQL: &str = r#"SELECT c.id AS case_id, r.reference AS reservation_reference, c."operatorId" AS operator_id,
        o.display_name AS operator_name, p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
        (SELECT g."fullName" FROM stay_reservation_items i
           JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
          WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS guest_name,
        (SELECT MAX(i."checkOut") FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS check_out,
        COALESCE(c."claimAmount", 0)::text AS claim_amount, c.state AS case_state,
        d.decision, d."approvedAmount"::text AS approved_amount, d."payoutState" AS payout_state,
        c."createdAt" AS opened_at, c."deadlineAt" AS deadline_at"#;

const SUMMARY_SQL: &str = r#"SELECT COUNT(*) AS total,
        COUNT(*) FILTER (WHERE d."caseId" IS NULL AND c.state IN ('OPEN', 'AWAITING_RESPONSE')) AS undecided,
        COALESCE(SUM(c."claimAmount"), 0)::text AS claimed,
        COALESCE(SUM(d."approvedAmount"), 0)::text AS approved,
        COALESCE(SUM(d."approvedAmount") FILTER (WHERE d."payoutState" = 'PENDING'), 0)::text AS payout_pending,
        COALESCE(SUM(d."approvedAmount") FILTER (WHERE d."payoutState" = 'PAID'), 0)::text AS paid"#;

fn push<'a>(builder: &mut QueryBuilder<'a, Postgres>, city: City, filter: &'a ClaimFilter, with_status: bool) {
    builder.push(" AND c.city_id = ").push_bind(city.code());
    if with_status {
        match filter.status.as_deref() {
            Some("UNDECIDED") => {
                builder.push(r#" AND d."caseId" IS NULL AND c.state IN ('OPEN', 'AWAITING_RESPONSE')"#);
            }
            Some(status) => {
                builder.push(" AND d.decision = ").push_bind(status);
            }
            None => {}
        }
    }
    if let Some(host_id) = filter.host_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        builder.push(r#" AND c."operatorId" = "#).push_bind(host_id);
    }
    if let Some(search) = filter.search.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        let pattern = format!("%{}%", escape_like(search));
        builder
            .push(" AND (r.reference ILIKE ")
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR o.display_name ILIKE "#)
            .push_bind(pattern)
            .push(r#" ESCAPE '\')"#);
    }
}

pub async fn list(kernel: &Kernel, actor: &Actor, filter: ClaimFilter) -> Result<Page<ClaimRow>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    ensure_status(filter.status.as_deref())?;
    let page = paging(filter.page, filter.page_size)?;
    let pool = kernel.db.pool();
    let mut total_builder = QueryBuilder::<Postgres>::new(format!("SELECT COUNT(*){FROM_SQL}"));
    push(&mut total_builder, actor.city, &filter, true);
    let total: i64 = total_builder.build_query_scalar().fetch_one(pool).await?;
    let mut builder = QueryBuilder::<Postgres>::new(format!("{SELECT_SQL}{FROM_SQL}"));
    push(&mut builder, actor.city, &filter, true);
    builder
        .push(r#" ORDER BY (d."caseId" IS NULL AND c.state IN ('OPEN', 'AWAITING_RESPONSE')) DESC, c."deadlineAt", c.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let items = builder.build_query_as::<ClaimRow>().fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}

pub async fn summary(kernel: &Kernel, actor: &Actor, filter: ClaimFilter) -> Result<ClaimSummary> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    ensure_status(filter.status.as_deref())?;
    let mut builder = QueryBuilder::<Postgres>::new(format!("{SUMMARY_SQL}{FROM_SQL}"));
    push(&mut builder, actor.city, &filter, false);
    Ok(builder.build_query_as::<ClaimSummary>().fetch_one(kernel.db.pool()).await?)
}
