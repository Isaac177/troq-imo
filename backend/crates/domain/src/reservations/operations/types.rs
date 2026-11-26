use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskInput {
    pub task_type: String,
    pub property_id: String,
    pub physical_unit_id: Option<String>,
    pub reservation_item_id: Option<String>,
    pub priority: Option<String>,
    #[serde(default)]
    pub checklist: Vec<String>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct TaskListQuery {
    pub property_id: String,
    pub state: Option<String>,
    pub task_type: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionInput {
    pub action: String,
    pub expected_version: i32,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskView {
    pub id: String,
    pub property_id: String,
    pub physical_unit_id: Option<String>,
    pub reservation_item_id: Option<String>,
    pub task_type: String,
    pub assignee_user_id: Option<String>,
    pub state: String,
    pub priority: String,
    #[schema(value_type = Vec<String>)]
    pub checklist: Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
