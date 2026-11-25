use kernel::city::City;
use kernel::storage::{key_for, Scope, Storage, StorageResult, Visibility};
use sha2::{Digest, Sha256};

use super::validation::ImageType;

#[derive(Debug, Clone, Copy)]
pub enum ImageFolder {
    Property,
    Template,
}

impl ImageFolder {
    fn path(self) -> &'static str {
        match self {
            Self::Property => "stays/properties",
            Self::Template => "stays/templates",
        }
    }
}

pub fn image_key(city: City, folder: ImageFolder, id: &str, image_type: ImageType) -> StorageResult<String> {
    let name = format!("{}.{}", hex::encode(Sha256::digest(id.as_bytes())), image_type.extension());
    key_for(&Scope::for_city(city), folder.path(), &name)
}

pub async fn upload_image(
    storage: &Storage,
    city: City,
    folder: ImageFolder,
    id: &str,
    image_type: ImageType,
    content: &[u8],
) -> StorageResult<String> {
    let key = image_key(city, folder, id, image_type)?;
    storage.put(Visibility::Private, &key, content.to_vec(), image_type.mime()).await?;
    Ok(key)
}

pub async fn image_url(storage: &Storage, key: &str) -> StorageResult<String> {
    storage.presign_get(Visibility::Private, key, None).await
}

pub async fn delete_image(storage: &Storage, key: &str) -> StorageResult<()> {
    storage.delete(Visibility::Private, key).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_keys_are_city_scoped_and_deterministic() {
        let first = image_key(City::GOMA, ImageFolder::Property, "media-1", ImageType::Png).unwrap();
        assert_eq!(first, image_key(City::GOMA, ImageFolder::Property, "media-1", ImageType::Png).unwrap());
        assert!(first.starts_with("goma/stays/properties/"));
        assert!(first.ends_with(".png"));
        assert_ne!(first, image_key(City::KIN, ImageFolder::Property, "media-1", ImageType::Png).unwrap());
        assert!(image_key(City::GOMA, ImageFolder::Template, "media-1", ImageType::Jpeg).unwrap().starts_with("goma/stays/templates/"));
    }
}
