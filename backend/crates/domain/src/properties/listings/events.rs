use kernel::error::Result;
use serde_json::{json, Value};
use sqlx::{PgConnection, Postgres, Transaction};

use crate::properties::events::owner_user;

pub async fn payload(
    tx: &mut Transaction<'_, Postgres>,
    property_id: &str,
    revision_id: Option<&str>,
    extra: Value,
) -> Result<Value> {
    let row: Option<(String, String, String)> = {
        let conn: &mut PgConnection = &mut *tx;
        sqlx::query_as(r#"SELECT "operatorId", "nameEn", "nameFr" FROM stay_properties WHERE id = $1"#)
            .bind(property_id)
            .fetch_optional(conn)
            .await?
    };
    let (host_id, name_en, name_fr) = row.unwrap_or_default();
    let owner = owner_user(tx, &host_id).await?;
    let mut value = json!({
        "propertyId": property_id,
        "revisionId": revision_id,
        "hostId": host_id,
        "ownerUserId": owner,
        "propertyName": { "en": name_en, "fr": name_fr },
    });
    if let (Some(base), Some(more)) = (value.as_object_mut(), extra.as_object()) {
        for (key, item) in more {
            base.insert(key.clone(), item.clone());
        }
    }
    Ok(value)
}
