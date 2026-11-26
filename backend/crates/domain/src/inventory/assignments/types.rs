use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignUnitInput {
    pub reservation_item_id: String,
    pub physical_unit_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoveUnitInput {
    pub physical_unit_id: String,
    pub from_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentView {
    pub id: String,
    pub reservation_item_id: String,
    pub physical_unit_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
}

pub const ASSIGNMENT_COLUMNS: &str = r#"id, "reservationItemId" AS reservation_item_id, "physicalUnitId" AS physical_unit_id,
    "startDate" AS start_date, "endDate" AS end_date, reason, "createdAt" AS created_at, "releasedAt" AS released_at"#;

#[derive(Debug, FromRow)]
pub struct ItemContext {
    pub item_id: String,
    pub reservation_id: String,
    pub property_id: String,
    pub unit_type_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub state: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitOptionView {
    pub id: String,
    pub local_code: String,
}
