use kernel::auth::Actor;
use kernel::permissions::names::{OPERATIONS_CHECKIN, OPERATIONS_HOUSEKEEPING, OPERATIONS_MAINTENANCE};
use kernel::{Error, Result};
use serde_json::json;

use crate::reservations::codes::{
    CHECKLIST_ITEM_TOO_LONG, CHECKLIST_TOO_LONG, TASK_ACTION_UNKNOWN, TASK_PRIORITY_UNKNOWN, TASK_STATE_UNKNOWN,
    TASK_TRANSITION_INVALID, TASK_TYPE_FORBIDDEN, TASK_TYPE_UNKNOWN,
};

pub const TASK_TYPES: [&str; 5] = ["CHECK_IN", "CHECK_OUT", "HOUSEKEEPING", "MAINTENANCE", "SERVICE_REQUEST"];
pub const TASK_STATES: [&str; 4] = ["OPEN", "IN_PROGRESS", "DONE", "CANCELLED"];
pub const PRIORITIES: [&str; 4] = ["LOW", "NORMAL", "HIGH", "URGENT"];
const MAX_CHECKLIST_ITEMS: usize = 50;
const MAX_CHECKLIST_ITEM_LENGTH: usize = 200;

pub fn granted(actor: &Actor) -> Vec<String> {
    actor.access.grants.iter().map(|grant| grant.permission.clone()).collect()
}

pub fn task_types_for(permissions: &[String]) -> Vec<&'static str> {
    let has = |permission: &str| permissions.iter().any(|granted| granted == permission);
    let mut types = Vec::new();
    if has(OPERATIONS_CHECKIN) {
        types.extend(["CHECK_IN", "CHECK_OUT"]);
    }
    if has(OPERATIONS_HOUSEKEEPING) {
        types.extend(["HOUSEKEEPING", "SERVICE_REQUEST"]);
    }
    if has(OPERATIONS_MAINTENANCE) {
        types.push("MAINTENANCE");
    }
    types
}

pub fn ensure_task_type_allowed(task_type: &str, permissions: &[String]) -> Result<()> {
    if task_types_for(permissions).contains(&task_type) {
        Ok(())
    } else {
        Err(Error::forbidden(TASK_TYPE_FORBIDDEN).with_params(json!({ "taskType": task_type })))
    }
}

pub fn ensure_choice(value: Option<&str>, allowed: &[&str], code: kernel::ErrorCode) -> Result<()> {
    match value {
        Some(value) if !allowed.contains(&value) => Err(Error::bad_request(code).with_params(json!({ "value": value }))),
        _ => Ok(()),
    }
}

pub fn validate_new(task_type: &str, priority: Option<&str>) -> Result<String> {
    ensure_choice(Some(task_type), &TASK_TYPES, TASK_TYPE_UNKNOWN)?;
    ensure_choice(priority, &PRIORITIES, TASK_PRIORITY_UNKNOWN)?;
    Ok(priority.unwrap_or("NORMAL").to_string())
}

pub fn ensure_state(state: Option<&str>) -> Result<()> {
    ensure_choice(state, &TASK_STATES, TASK_STATE_UNKNOWN)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Start,
    Complete,
    Cancel,
    AssignMe,
    Unassign,
}

impl Action {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "START" => Ok(Self::Start),
            "COMPLETE" => Ok(Self::Complete),
            "CANCEL" => Ok(Self::Cancel),
            "ASSIGN_ME" => Ok(Self::AssignMe),
            "UNASSIGN" => Ok(Self::Unassign),
            other => Err(Error::bad_request(TASK_ACTION_UNKNOWN).with_params(json!({ "action": other }))),
        }
    }
}

pub fn next_state(state: &str, action: Action) -> Result<&'static str> {
    let open = matches!(state, "OPEN" | "IN_PROGRESS");
    match action {
        Action::Start if state == "OPEN" => Ok("IN_PROGRESS"),
        Action::Complete if open => Ok("DONE"),
        Action::Cancel if open => Ok("CANCELLED"),
        Action::AssignMe | Action::Unassign if open => Ok(static_state(state)),
        _ => Err(Error::conflict(TASK_TRANSITION_INVALID).with_params(json!({ "state": state, "action": format!("{action:?}") }))),
    }
}

pub fn assignee_after(action: Action, actor: &str, current: Option<&str>) -> Option<String> {
    match action {
        Action::AssignMe => Some(actor.to_string()),
        Action::Unassign => None,
        Action::Start if current.is_none() => Some(actor.to_string()),
        _ => current.map(str::to_string),
    }
}

fn static_state(state: &str) -> &'static str {
    TASK_STATES.iter().copied().find(|candidate| *candidate == state).unwrap_or("OPEN")
}

pub fn checklist(items: Vec<String>) -> Result<serde_json::Value> {
    let cleaned: Vec<String> = items.into_iter().map(|item| item.trim().to_string()).filter(|item| !item.is_empty()).collect();
    if cleaned.len() > MAX_CHECKLIST_ITEMS {
        return Err(Error::bad_request(CHECKLIST_TOO_LONG).with_params(json!({ "max": MAX_CHECKLIST_ITEMS })));
    }
    if cleaned.iter().any(|item| item.chars().count() > MAX_CHECKLIST_ITEM_LENGTH) {
        return Err(Error::bad_request(CHECKLIST_ITEM_TOO_LONG).with_params(json!({ "max": MAX_CHECKLIST_ITEM_LENGTH })));
    }
    Ok(json!(cleaned))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perms(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn checkin_permission_grants_only_checkin_tasks() {
        assert_eq!(task_types_for(&perms(&[OPERATIONS_CHECKIN])), vec!["CHECK_IN", "CHECK_OUT"]);
    }

    #[test]
    fn housekeeping_permission_grants_upkeep_tasks() {
        assert_eq!(task_types_for(&perms(&[OPERATIONS_HOUSEKEEPING])), vec!["HOUSEKEEPING", "SERVICE_REQUEST"]);
    }

    #[test]
    fn maintenance_permission_grants_maintenance_tasks() {
        assert_eq!(task_types_for(&perms(&[OPERATIONS_MAINTENANCE])), vec!["MAINTENANCE"]);
    }

    #[test]
    fn no_operations_permission_grants_nothing() {
        assert!(task_types_for(&perms(&["properties.read"])).is_empty());
        assert!(ensure_task_type_allowed("CHECK_IN", &perms(&["properties.read"])).is_err());
    }

    #[test]
    fn open_task_moves_through_start_and_complete() {
        assert_eq!(next_state("OPEN", Action::Start).unwrap(), "IN_PROGRESS");
        assert_eq!(next_state("IN_PROGRESS", Action::Complete).unwrap(), "DONE");
    }

    #[test]
    fn terminal_tasks_accept_no_action() {
        assert!(next_state("DONE", Action::Start).is_err());
        assert!(next_state("CANCELLED", Action::Complete).is_err());
        assert!(next_state("DONE", Action::AssignMe).is_err());
    }

    #[test]
    fn start_is_only_from_open() {
        assert!(next_state("IN_PROGRESS", Action::Start).is_err());
    }

    #[test]
    fn assignment_follows_the_action() {
        assert_eq!(assignee_after(Action::AssignMe, "u1", None), Some("u1".into()));
        assert_eq!(assignee_after(Action::Unassign, "u1", Some("u2")), None);
        assert_eq!(assignee_after(Action::Start, "u1", None), Some("u1".into()));
        assert_eq!(assignee_after(Action::Start, "u1", Some("u2")), Some("u2".into()));
    }

    #[test]
    fn unknown_action_is_rejected() {
        assert!(Action::parse("DELETE").is_err());
    }

    #[test]
    fn checklists_are_trimmed_and_bounded() {
        assert_eq!(checklist(vec![" a ".into(), " ".into()]).unwrap(), json!(["a"]));
        assert!(checklist(vec!["x".into(); MAX_CHECKLIST_ITEMS + 1]).is_err());
    }
}
