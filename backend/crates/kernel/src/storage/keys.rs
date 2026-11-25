use uuid::Uuid;

use crate::city::City;

use super::error::{StorageError, StorageResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    Global,
    City(String),
}

impl Scope {
    pub fn for_city(city: City) -> Self {
        Scope::City(city.code().to_ascii_lowercase())
    }

    pub fn prefix(&self) -> &str {
        match self {
            Scope::Global => "global",
            Scope::City(code) => code,
        }
    }
}

pub fn unique_name(extension: &str) -> String {
    let extension = extension.trim_start_matches('.').to_ascii_lowercase();
    if extension.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        format!("{}.{}", Uuid::new_v4(), extension)
    }
}

pub fn sanitize_segment(value: &str) -> String {
    let cleaned: String = value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_' { c } else { '-' })
        .collect();
    if cleaned.is_empty() {
        "misc".to_string()
    } else {
        cleaned
    }
}

pub fn key_for(scope: &Scope, kind: &str, name: &str) -> StorageResult<String> {
    let kind = kind.trim_matches('/');
    let kind_valid = !kind.is_empty()
        && kind
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_' || c == '/');
    if !kind_valid || name.is_empty() {
        return Err(StorageError::InvalidKey);
    }
    let key = format!("{}/{}/{}", scope.prefix(), kind, name);
    validate_key(&key)?;
    Ok(key)
}

pub fn validate_key(key: &str) -> StorageResult<()> {
    let valid = !key.is_empty()
        && key.len() <= 512
        && !key.starts_with('/')
        && !key.contains('\\')
        && key.split('/').all(|part| !part.is_empty() && part != "." && part != "..");
    if valid {
        Ok(())
    } else {
        Err(StorageError::InvalidKey)
    }
}

pub fn key_in_scopes(key: &str, scopes: &[Scope]) -> bool {
    validate_key(key).is_ok()
        && scopes.iter().any(|scope| {
            key.strip_prefix(scope.prefix())
                .is_some_and(|rest| rest.starts_with('/'))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_city_and_global_keys() {
        let goma = Scope::for_city(City::GOMA);
        assert_eq!(key_for(&goma, "stays/properties", "a.png").unwrap(), "goma/stays/properties/a.png");
        assert_eq!(key_for(&Scope::Global, "categories", "a.png").unwrap(), "global/categories/a.png");
    }

    #[test]
    fn rejects_unsafe_input() {
        assert!(key_for(&Scope::Global, "../x", "a.png").is_err());
        assert!(validate_key("goma/../x").is_err());
        assert!(validate_key("/goma/x").is_err());
    }

    #[test]
    fn checks_scope_membership() {
        let scopes = [Scope::for_city(City::GOMA), Scope::Global];
        assert!(key_in_scopes("goma/reviews/a.png", &scopes));
        assert!(key_in_scopes("global/brands/a.png", &scopes));
        assert!(!key_in_scopes("kin/reviews/a.png", &scopes));
        assert!(!key_in_scopes("gomax/reviews/a.png", &scopes));
    }

    #[test]
    fn unique_names_keep_the_extension() {
        assert!(unique_name(".PNG").ends_with(".png"));
        assert_eq!(sanitize_segment(" A b/"), "a-b-");
        assert_eq!(sanitize_segment(""), "misc");
    }
}
