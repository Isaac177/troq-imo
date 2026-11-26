use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::storage::Storage;
use sqlx::PgPool;

use super::queries;
use super::types::MediaContentView;
use crate::guests::messages::MEDIA_NOT_FOUND;
use crate::properties::media::image_url;

pub async fn media_content(
    pool: &PgPool,
    storage: &Storage,
    city: City,
    media_id: &str,
    locale: &str,
) -> Result<MediaContentView> {
    let media = match queries::public_media(pool, city, media_id, locale).await? {
        Some(media) => media,
        None => queries::public_template_media(pool, city, media_id, locale)
            .await?
            .ok_or_else(|| Error::not_found(MEDIA_NOT_FOUND))?,
    };
    let url = image_url(storage, &media.storage_key).await?;
    Ok(MediaContentView { media_id: media.id, mime_type: media.mime_type, alt: media.alt, url })
}
