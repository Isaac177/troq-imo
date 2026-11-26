use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::billing::amounts::wire;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MineSummary {
    pub balance: String,
    pub rate_bps: i32,
    pub enabled: bool,
    pub low: bool,
    pub low_threshold: i32,
}

#[derive(Debug, FromRow)]
pub struct LedgerRecord {
    pub id: String,
    pub entry_type: String,
    pub amount_minor: i64,
    pub balance_after_minor: i64,
    pub reservation_id: Option<String>,
    pub reservation_reference: Option<String>,
    pub reason_code: Option<String>,
    pub payment_ref: Option<String>,
    pub note: Option<String>,
    pub created_by_user_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MineLedgerRow {
    pub id: String,
    pub entry_type: String,
    pub amount: String,
    pub balance_after: String,
    pub reservation_id: Option<String>,
    pub reservation_reference: Option<String>,
    pub reason_code: Option<String>,
    pub payment_ref: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<LedgerRecord> for MineLedgerRow {
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
            created_at: record.created_at,
        }
    }
}
