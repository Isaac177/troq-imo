use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::types::{StatementInput, StatementRow};
use crate::reservations::codes::{CLAIM_NOT_FOUND, STATEMENT_BODY_INVALID, STATEMENT_PARTY_INVALID};

const MAX_CHARS: usize = 2000;

pub fn clean(input: &StatementInput) -> Result<String> {
    if input.party != "HOST" && input.party != "GUEST" {
        return Err(Error::bad_request(STATEMENT_PARTY_INVALID));
    }
    let body = input.body.trim().to_string();
    if body.is_empty() || body.chars().count() > MAX_CHARS {
        return Err(Error::bad_request(STATEMENT_BODY_INVALID).with_params(json!({ "max": MAX_CHARS })));
    }
    Ok(body)
}

pub async fn add(kernel: &Kernel, actor: &Actor, case_id: &str, input: StatementInput) -> Result<StatementRow> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let body = clean(&input)?;
    let exists: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_cases WHERE id = $1 AND city_id = $2 AND "caseType" = 'DAMAGE_CLAIM')"#,
    )
    .bind(case_id)
    .bind(actor.city.code())
    .fetch_one(pool)
    .await?;
    if !exists {
        return Err(Error::not_found(CLAIM_NOT_FOUND));
    }
    let id = new_id("claim-statement");
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, StatementRow>(
        r#"INSERT INTO stay_claim_statements (id, "caseId", party, body, "recordedByUserId")
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, party, body, "createdAt" AS created_at"#,
    )
    .bind(&id)
    .bind(case_id)
    .bind(&input.party)
    .bind(&body)
    .bind(actor.user_id().as_str())
    .fetch_one(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "claim.statement_recorded",
            resource_type: "case",
            resource_id: case_id,
            reason: Some(input.party.as_str()),
            change: json!({ "statementId": id }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statements_need_a_known_party_and_a_bounded_body() {
        let input = |party: &str, body: &str| StatementInput { party: party.into(), body: body.into() };
        assert!(clean(&input("OTHER", "x")).is_err());
        assert!(clean(&input("HOST", "  ")).is_err());
        assert!(clean(&input("GUEST", &"x".repeat(MAX_CHARS + 1))).is_err());
        assert_eq!(clean(&input("HOST", " text ")).unwrap(), "text");
    }
}
