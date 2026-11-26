use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Result};
use serde_json::json;
use sqlx::PgPool;

use super::credit_types::{AdjustInput, BalanceView, TopupInput};
use crate::billing::amounts::wire;
use crate::billing::credits::ledger::{self, EntryType, NewEntry};
use crate::billing::messages::{DIRECTION_UNKNOWN, PAYMENT_REF_REQUIRED};
use crate::billing::scope;
use crate::billing::validators::{
    check_note, check_payment_ref, clean, ensure_adjust_reason, ensure_topup_reason, parse_amount_minor,
    PROMOTIONAL_REASON,
};

pub async fn topup(pool: &PgPool, actor: &Actor, host_id: &str, input: TopupInput) -> Result<BalanceView> {
    scope::require_staff(actor)?;
    ensure_topup_reason(&input.reason_code)?;
    let amount_minor = parse_amount_minor(&input.amount)?;
    let payment_ref = clean(&input.payment_ref);
    let note = clean(&input.note);
    check_note(note)?;
    check_payment_ref(payment_ref)?;
    if input.reason_code != PROMOTIONAL_REASON && payment_ref.is_none() {
        return Err(Error::bad_request(PAYMENT_REF_REQUIRED));
    }
    scope::ensure_host(pool, host_id, actor.city).await?;
    let mut tx = pool.begin().await?;
    let balance = ledger::apply(
        &mut tx,
        NewEntry {
            city: actor.city,
            host_id,
            entry_type: EntryType::Topup,
            amount_minor,
            reservation_id: None,
            reason_code: Some(input.reason_code.as_str()),
            payment_ref,
            note,
            created_by: Some(actor.user_id().as_str()),
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "credit.topup",
            resource_type: "operator",
            resource_id: host_id,
            reason: Some(input.reason_code.as_str()),
            change: json!({ "amountMinor": amount_minor, "balanceAfterMinor": balance, "paymentRef": payment_ref }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(BalanceView { host_id: host_id.to_string(), balance: wire(balance) })
}

pub async fn adjust(pool: &PgPool, actor: &Actor, host_id: &str, input: AdjustInput) -> Result<BalanceView> {
    scope::require_staff(actor)?;
    ensure_adjust_reason(&input.reason_code)?;
    let magnitude = parse_amount_minor(&input.amount)?;
    let amount_minor = match input.direction.as_str() {
        "CREDIT" => magnitude,
        "DEBIT" => -magnitude,
        other => return Err(Error::bad_request(DIRECTION_UNKNOWN).with_params(json!({ "direction": other }))),
    };
    let note = clean(&input.note);
    check_note(note)?;
    scope::ensure_host(pool, host_id, actor.city).await?;
    let mut tx = pool.begin().await?;
    let balance = ledger::apply(
        &mut tx,
        NewEntry {
            city: actor.city,
            host_id,
            entry_type: EntryType::Adjustment,
            amount_minor,
            reservation_id: None,
            reason_code: Some(input.reason_code.as_str()),
            payment_ref: None,
            note,
            created_by: Some(actor.user_id().as_str()),
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "credit.adjust",
            resource_type: "operator",
            resource_id: host_id,
            reason: Some(input.reason_code.as_str()),
            change: json!({ "amountMinor": amount_minor, "direction": input.direction, "balanceAfterMinor": balance }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(BalanceView { host_id: host_id.to_string(), balance: wire(balance) })
}
