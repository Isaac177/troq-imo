use kernel::storage::Visibility;
use kernel::{City, Error, Kernel, Result};
use serde_json::Value;

use super::build::{build, mail_copy};
use super::kind::{DocumentKind, Locale};
use super::payload::Payload;
use super::pipeline::Generated;
use super::store::{self, internal, StoreRequest};
use crate::notifications::config::NotificationsConfig;

pub struct Issued {
    pub generated: Generated,
    pub subject: String,
    pub body: String,
}

fn text<'a>(snapshot: &'a Value, key: &str) -> Option<&'a str> {
    snapshot.get(key).and_then(Value::as_str).filter(|value| !value.is_empty())
}

pub async fn issue(
    kernel: &Kernel,
    config: &NotificationsConfig,
    kind: DocumentKind,
    city: City,
    snapshot: &Value,
    locale: Locale,
    delivery_id: &str,
) -> Result<Option<Issued>> {
    let Some(payload) = Payload::parse(kind, snapshot, locale).map_err(internal)? else {
        return Ok(None);
    };
    let pool = kernel.db.pool();
    let entity_id = payload.entity_id().to_string();
    let root = config.assets_dir.as_path();
    let brand = config.product_name.as_str();
    if let Some(existing) = store::find(&mut *pool.acquire().await?, kind, &entity_id).await? {
        if existing.city != city.code() {
            return Err(Error::internal("document city does not match delivery"));
        }
        let Some(source) = existing.source_delivery_id.as_deref() else {
            return Err(Error::internal("existing document has no delivery source; reconciliation required"));
        };
        if source != delivery_id {
            return Ok(None);
        }
        let bytes = kernel.storage.get_bytes(Visibility::Private, &existing.storage_key).await?;
        let locale = Locale::parse(Some(&existing.locale));
        let (subject, body) = mail_copy(root, brand, kind, locale, payload.reference()).map_err(internal)?;
        let generated = Generated { bytes: bytes.to_vec(), sha256: existing.sha256, page_count: existing.page_count as usize, file_name: existing.file_name };
        return Ok(Some(Issued { generated, subject, body }));
    }
    let engine = super::engine(config)?;
    let Some(built) = build(engine, root, brand, kind, snapshot, locale).await.map_err(internal)? else {
        return Ok(None);
    };
    let host_id = text(snapshot, "hostId").or_else(|| text(snapshot, "operatorId"));
    let request = StoreRequest {
        city,
        kind,
        locale: locale.code(),
        entity_id: &entity_id,
        guest_user_id: text(snapshot, "guestUserId"),
        host_id,
        created_by: "notifications",
        source_delivery_id: delivery_id,
    };
    let key = store::upload(&kernel.storage, &built.generated, &request).await?;
    let mut tx = pool.begin().await?;
    store::record(&mut tx, &built.generated, &key, &request).await?;
    tx.commit().await?;
    let (subject, body) = mail_copy(root, brand, kind, locale, &built.reference).map_err(internal)?;
    Ok(Some(Issued { generated: built.generated, subject, body }))
}
