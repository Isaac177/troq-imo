use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::billing::amounts::wire;
use crate::billing::credits::types::LedgerRecord;

#[derive(Debug, Default)]
pub struct AccountFilter {
    pub host_id: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TopupInput {
    pub amount: String,
    pub reason_code: String,
    pub payment_ref: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustInput {
    pub amount: String,
    pub direction: String,
    pub reason_code: String,
    pub note: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct AccountRecord {
    pub operator_id: String,
    pub operator_name: String,
    pub rate_bps: i32,
    pub balance_minor: i64,
    pub enabled: bool,
    pub low: bool,
    pub last_top_up_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountRow {
    pub host_id: String,
    pub host_name: String,
    pub rate_bps: i32,
    pub balance: String,
    pub enabled: bool,
    pub low: bool,
    pub last_top_up_at: Option<DateTime<Utc>>,
}

impl From<AccountRecord> for AccountRow {
    fn from(record: AccountRecord) -> Self {
        Self {
            host_id: record.operator_id,
            host_name: record.operator_name,
            rate_bps: record.rate_bps,
            balance: wire(record.balance_minor),
            enabled: record.enabled,
            low: record.low,
            last_top_up_at: record.last_top_up_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreditSummary {
    pub accounts: i64,
    pub disabled: i64,
    pub low: i64,
    pub total_balance: String,
    pub topped_up_30d: String,
    pub commission_taken_30d: String,
    pub refunded_30d: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreditDetail {
    pub host_id: String,
    pub rate_bps: i32,
    pub balance: String,
    pub enabled: bool,
    pub low: bool,
    pub low_threshold: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BalanceView {
    pub host_id: String,
    pub balance: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminLedgerRow {
    pub id: String,
    pub entry_type: String,
    pub amount: String,
    pub balance_after: String,
    pub reservation_id: Option<String>,
    pub reservation_reference: Option<String>,
    pub reason_code: Option<String>,
    pub payment_ref: Option<String>,
    pub note: Option<String>,
    pub created_by_user_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<LedgerRecord> for AdminLedgerRow {
    fn from(record: LedgerRecord) -> Self {
        Self {
            id: record.id,
            entry_type: record.entry_type,
            amount: wire(record.amount_minor),
            balance_after: wire(record.balance_after_minor),
            reservation_id: record.reservation_id,
            reservation_reference: record.reservation_reference,
            reason_code: record.reason_code,
            payment_ref: record.payment_ref,
            note: record.note,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        }
    }
}
