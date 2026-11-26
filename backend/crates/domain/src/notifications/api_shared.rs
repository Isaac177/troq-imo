use axum::body::Body;
use axum::http::{header, HeaderValue};
use axum::response::Response;
use kernel::auth::Actor;
use kernel::error::codes::MALFORMED_REQUEST;
use kernel::permissions::names;
use kernel::{Error, Kernel, Page, Result};

use super::api_queries::{paging, DocumentsQuery, ListQuery, PreviewQuery};
use super::documents::{self, DocumentKind, DocumentScope, Locale};
use super::config::NotificationsConfig;
use super::inbox_read::{self, locale};
use super::prefs::{self, Channels};
use super::types::{DocumentView, MarkedCount, NotificationView, PreferenceList, PreferenceView, PushTokenInput, PushTokenRemoval, UnreadCount};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audience {
    Guest,
    Host,
    Admin,
}

pub async fn list(kernel: &Kernel, actor: &Actor, query: ListQuery) -> Result<Page<NotificationView>> {
    let language = locale(query.locale.as_deref())?;
    let page = paging(query.page, query.page_size);
    inbox_read::list(kernel.db.pool(), actor.user_id().as_str(), query.unread_only.unwrap_or(false), language, page).await
}

pub async fn unread(kernel: &Kernel, actor: &Actor) -> Result<UnreadCount> {
    Ok(UnreadCount { unread: inbox_read::unread_count(kernel.db.pool(), actor.user_id().as_str()).await? })
}

pub async fn read_one(kernel: &Kernel, actor: &Actor, id: &str) -> Result<MarkedCount> {
    inbox_read::mark_read(kernel.db.pool(), actor.user_id().as_str(), id).await?;
    Ok(MarkedCount { updated: 1 })
}

pub async fn read_all(kernel: &Kernel, actor: &Actor) -> Result<MarkedCount> {
    inbox_read::mark_all_read(kernel.db.pool(), actor.user_id().as_str()).await
}

pub async fn preferences(kernel: &Kernel, actor: &Actor) -> Result<PreferenceList> {
    Ok(PreferenceList { items: prefs::list(kernel.db.pool(), actor.user_id().as_str()).await? })
}

pub async fn set_preference(kernel: &Kernel, actor: &Actor, input: PreferenceView) -> Result<PreferenceView> {
    let channels = Channels { in_app: input.in_app, push: input.push, email: input.email };
    prefs::upsert(kernel.db.pool(), actor.user_id().as_str(), &input.event_key, channels).await?;
    Ok(input)
}

pub async fn register_token(kernel: &Kernel, actor: &Actor, input: PushTokenInput) -> Result<MarkedCount> {
    super::channels::push::register(kernel.db.pool(), actor.user_id().as_str(), &input).await?;
    Ok(MarkedCount { updated: 1 })
}

pub async fn remove_token(kernel: &Kernel, actor: &Actor, input: PushTokenRemoval) -> Result<MarkedCount> {
    super::channels::push::unregister(kernel.db.pool(), actor.user_id().as_str(), &input.token).await?;
    Ok(MarkedCount { updated: 1 })
}

fn host_scope(actor: &Actor) -> Result<Vec<kernel::ids::HostId>> {
    let (hosts, _) = actor.access.scoped_access(names::RESERVATIONS_READ);
    if hosts.is_empty() {
        return Err(Error::permission_missing(names::RESERVATIONS_READ));
    }
    Ok(hosts)
}

pub async fn documents(kernel: &Kernel, actor: &Actor, audience: Audience, query: DocumentsQuery) -> Result<Page<DocumentView>> {
    let page = paging(query.page, query.page_size);
    let (kind, entity) = (query.kind.as_deref(), query.entity_id.as_deref());
    let pool = kernel.db.pool();
    match audience {
        Audience::Guest => documents::list(pool, &DocumentScope::Guest(actor.user_id().as_str()), kind, entity, page).await,
        Audience::Host => {
            let hosts = host_scope(actor)?;
            documents::list(pool, &DocumentScope::Host(&hosts), kind, entity, page).await
        }
        Audience::Admin => {
            actor.require_platform(names::AUDIT_READ)?;
            documents::list(pool, &DocumentScope::Admin, kind, entity, page).await
        }
    }
}

fn pdf(file_name: &str, bytes: Vec<u8>) -> Response {
    let mut response = Response::new(Body::from(bytes));
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("private, no-store"));
    let disposition = format!("inline; filename=\"{}\"", file_name.replace('"', ""));
    if let Ok(value) = HeaderValue::from_str(&disposition) {
        headers.insert(header::CONTENT_DISPOSITION, value);
    }
    response
}

pub async fn download(kernel: &Kernel, actor: &Actor, audience: Audience, id: &str) -> Result<Response> {
    let user = actor.user_id().as_str();
    let (name, bytes) = match audience {
        Audience::Guest => documents::fetch(kernel, &DocumentScope::Guest(user), id, user).await?,
        Audience::Host => {
            let hosts = host_scope(actor)?;
            documents::fetch(kernel, &DocumentScope::Host(&hosts), id, user).await?
        }
        Audience::Admin => {
            actor.require_platform(names::AUDIT_READ)?;
            documents::fetch(kernel, &DocumentScope::Admin, id, user).await?
        }
    };
    Ok(pdf(&name, bytes))
}

pub async fn preview(actor: &Actor, kind: &str, query: PreviewQuery) -> Result<Response> {
    actor.require_platform(names::AUDIT_READ)?;
    let kind = DocumentKind::parse(kind).ok_or_else(|| Error::bad_request(super::codes::DOCUMENT_KIND_INVALID))?;
    let locale = match query.locale.as_deref().map(str::trim) {
        Some("en") | None => Locale::En,
        Some("fr") => Locale::Fr,
        Some(_) => return Err(Error::bad_request(super::codes::LOCALE_INVALID)),
    };
    let config = NotificationsConfig::from_env();
    let (name, bytes) = documents::preview(&config, kind, locale).await?;
    Ok(pdf(&name, bytes))
}

pub fn malformed() -> Error {
    Error::bad_request(MALFORMED_REQUEST)
}
