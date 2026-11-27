mod admin;
mod admin_ops;
pub mod extract;
mod guest;
mod guest_cases;
mod host;
mod host_cases;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;

use extract::AppState;

const EVIDENCE_BODY_LIMIT: usize = 8 * 1024 * 1024;

pub fn guest_routes<S: AppState>() -> Router<S> {
    Router::new()
        .route("/reservations", post(guest::hold::<S>))
        .route("/reservations/mine", get(guest::mine::<S>))
        .route("/reservations/:reservation_id", get(guest::get::<S>))
        .route("/reservations/:reservation_id/cancellation", get(guest::cancellation::<S>))
        .route("/reservations/:reservation_id/cancel", post(guest::cancel::<S>))
        .route("/reservations/:reservation_id/withdraw", post(guest::withdraw::<S>))
        .route("/reservations/:reservation_id/amendments", get(guest::amendments::<S>).post(guest::propose::<S>))
        .route("/amendments/:amendment_id/accept", post(guest::accept::<S>))
        .route("/amendments/:amendment_id/decline", post(guest::decline::<S>))
        .route("/reservations/:reservation_id/messages", get(guest::messages_list::<S>).post(guest::messages_send::<S>))
        .route("/cases", post(guest_cases::open::<S>))
        .route("/cases/mine", get(guest_cases::mine::<S>))
        .route("/cases/evidence/:evidence_id", get(guest_cases::evidence_get::<S>))
        .route("/cases/:case_id", get(guest_cases::get::<S>))
        .route("/cases/:case_id/close", post(guest_cases::close::<S>))
        .route(
            "/cases/:case_id/evidence",
            get(guest_cases::evidence_list::<S>).post(guest_cases::evidence_add::<S>).layer(DefaultBodyLimit::max(EVIDENCE_BODY_LIMIT)),
        )
}

pub fn host_routes<S: AppState>() -> Router<S> {
    Router::new()
        .route("/message-conversations", get(host::message_conversations::<S>))
        .route("/reservations", get(host::list::<S>))
        .route("/reservations/export", get(host::export::<S>))
        .route("/reservations/:reservation_id", get(host::get::<S>))
        .route("/reservations/:reservation_id/decision", post(host::decide::<S>))
        .route("/reservations/:reservation_id/special-request", post(host::special_request::<S>))
        .route("/reservations/:reservation_id/check-in", post(host::check_in::<S>))
        .route("/reservations/:reservation_id/check-out", post(host::check_out::<S>))
        .route("/reservations/:reservation_id/no-show", post(host::no_show::<S>))
        .route("/reservations/:reservation_id/cancel", post(host::cancel::<S>))
        .route("/reservations/:reservation_id/amendments", post(host::amend::<S>))
        .route("/reservations/:reservation_id/messages", get(host::messages_list::<S>).post(host::messages_send::<S>))
        .route("/reservations/:reservation_id/messages/read", post(host::messages_mark_read::<S>))
        .route("/cases", get(host_cases::list::<S>).post(host_cases::open::<S>))
        .route("/cases/evidence/:evidence_id", get(host_cases::evidence_get::<S>))
        .route("/cases/:case_id", get(host_cases::get::<S>))
        .route("/cases/:case_id/close", post(host_cases::close::<S>))
        .route(
            "/cases/:case_id/evidence",
            get(host_cases::evidence_list::<S>).post(host_cases::evidence_add::<S>).layer(DefaultBodyLimit::max(EVIDENCE_BODY_LIMIT)),
        )
        .route("/tasks", get(host_cases::tasks_list::<S>).post(host_cases::tasks_create::<S>))
        .route("/tasks/:task_id", get(host_cases::tasks_get::<S>))
        .route("/tasks/:task_id/transition", post(host_cases::tasks_transition::<S>))
}

pub fn admin_routes<S: AppState>() -> Router<S> {
    Router::new()
        .route("/cases", get(admin::cases_list::<S>).post(admin::cases_open::<S>))
        .route("/cases/evidence/:evidence_id", get(admin::cases_evidence::<S>))
        .route("/cases/assignees", get(admin::cases_assignees::<S>))
        .route("/cases/:case_id", get(admin::cases_get::<S>))
        .route("/cases/:case_id/close", post(admin::cases_close::<S>))
        .route("/cases/:case_id/assign", post(admin::cases_assign::<S>))
        .route("/cases/:case_id/escalate", post(admin::cases_escalate::<S>))
        .route("/reservations/:reservation_id/cancel-preview", get(admin::cancel_preview::<S>))
        .route("/reservations/:reservation_id/cancel", post(admin::reservation_cancel::<S>))
        .route("/tasks", get(admin_ops::tasks_list::<S>).post(admin_ops::tasks_create::<S>))
        .route("/tasks/:task_id", get(admin_ops::tasks_get::<S>))
        .route("/tasks/:task_id/transition", post(admin_ops::tasks_transition::<S>))
        .route("/claims", get(admin_ops::claims_list::<S>))
        .route("/claims/summary", get(admin_ops::claims_summary::<S>))
        .route("/claims/:case_id", get(admin_ops::claims_get::<S>))
        .route("/claims/:case_id/statement", post(admin_ops::claims_statement::<S>))
        .route("/claims/:case_id/decision", post(admin_ops::claims_decide::<S>))
        .route("/claims/:case_id/payout", post(admin_ops::claims_payout::<S>))
}

#[derive(OpenApi)]
#[openapi(paths(
    guest::hold, guest::mine, guest::get, guest::cancellation, guest::cancel, guest::withdraw, guest::amendments,
    guest::propose, guest::accept, guest::decline, guest::messages_list, guest::messages_send,
    guest_cases::open, guest_cases::mine, guest_cases::get, guest_cases::close, guest_cases::evidence_list,
    guest_cases::evidence_add, guest_cases::evidence_get,
    host::list, host::export, host::get, host::decide, host::special_request, host::check_in, host::check_out,
    host::no_show, host::cancel, host::amend, host::messages_list, host::messages_send, host::messages_mark_read, host::message_conversations,
    host_cases::list, host_cases::open, host_cases::get, host_cases::close, host_cases::evidence_list,
    host_cases::evidence_add, host_cases::evidence_get, host_cases::tasks_list, host_cases::tasks_create,
    host_cases::tasks_get, host_cases::tasks_transition,
    admin::cases_list, admin::cases_open, admin::cases_assignees, admin::cases_get, admin::cases_close, admin::cases_assign,
    admin::cases_escalate, admin::cases_evidence, admin::cancel_preview, admin::reservation_cancel,
    admin_ops::tasks_list, admin_ops::tasks_create, admin_ops::tasks_get, admin_ops::tasks_transition,
    admin_ops::claims_list, admin_ops::claims_summary, admin_ops::claims_get, admin_ops::claims_statement,
    admin_ops::claims_decide, admin_ops::claims_payout
))]
struct ReservationsDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    ReservationsDoc::openapi()
}
