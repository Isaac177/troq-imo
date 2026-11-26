mod api_admin;
mod api_guest;
mod api_host;
mod api_queries;
mod api_shared;
mod catalog;
mod channels;
mod codes;
mod config;
mod contacts;
mod copy;
mod deliveries;
mod deliver;
mod deliver_document;
mod deliver_notice;
mod dispatch;
mod documents;
mod event_data;
mod handle;
mod inbox_read;
mod inbox_write;
mod invitation;
mod links;
mod notice;
mod prefs;
mod property_events;
mod recipients;
mod registration_otp;
mod render;
mod reservation_events;
mod types;

use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

pub use catalog::{find as find_event, EventDef, EVENTS};
pub use codes::{
    DOCUMENT_KIND_INVALID, DOCUMENT_NOT_FOUND, LOCALE_INVALID, NOTIFICATION_NOT_FOUND, PREFERENCE_KEY_INVALID,
    PUSH_TOKEN_INVALID,
};
pub use config::NotificationsConfig;
pub use registration_otp::{send_registration_otp, RegistrationChannel};
pub use contacts::{Contact, ContactDirectory};
pub use copy::{find as find_copy, Template};
pub use deliver::deliver_pending;
pub use dispatch::EventContext;
pub use documents::{DocumentKind, Locale};
pub use handle::{accepts, document_for, handle_event};
pub use types::{DocumentView, NotificationView, PreferenceView};

pub fn public_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
}

pub fn guest_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    api_guest::routes::<S>()
}

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    api_host::routes::<S>()
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    api_admin::routes::<S>()
}

#[derive(OpenApi)]
#[openapi(
    paths(
        api_guest::list,
        api_guest::unread_count,
        api_guest::mark_read,
        api_guest::mark_all_read,
        api_guest::preferences,
        api_guest::set_preference,
        api_guest::register_token,
        api_guest::remove_token,
        api_guest::documents,
        api_guest::document_download,
        api_host::list,
        api_host::unread_count,
        api_host::mark_read,
        api_host::mark_all_read,
        api_host::preferences,
        api_host::set_preference,
        api_host::register_token,
        api_host::remove_token,
        api_host::documents,
        api_host::document_download,
        api_admin::list,
        api_admin::unread_count,
        api_admin::mark_read,
        api_admin::mark_all_read,
        api_admin::preferences,
        api_admin::set_preference,
        api_admin::register_token,
        api_admin::remove_token,
        api_admin::documents,
        api_admin::document_download,
        api_admin::document_preview
    ),
    components(schemas(
        types::NotificationView,
        types::UnreadCount,
        types::MarkedCount,
        types::PreferenceView,
        types::PreferenceList,
        types::PushTokenInput,
        types::PushTokenRemoval,
        types::DocumentView
    ))
)]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
