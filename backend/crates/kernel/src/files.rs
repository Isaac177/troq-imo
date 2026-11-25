use crate::error::codes::FILE_NAME_INVALID;
use crate::error::{Error, Result};

const MAX_FILE_NAME_CHARS: usize = 200;

pub fn clean_file_name(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.chars().count() > MAX_FILE_NAME_CHARS
        || trimmed.contains('/')
        || trimmed.contains('\\')
    {
        return Err(Error::bad_request(FILE_NAME_INVALID)
            .with_params(serde_json::json!({ "maxLength": MAX_FILE_NAME_CHARS })));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_plain_names_trimmed() {
        assert_eq!(clean_file_name("  villa.png ").unwrap(), "villa.png");
    }

    #[test]
    fn rejects_paths_empty_and_long_names() {
        assert!(clean_file_name("a/b.png").is_err());
        assert!(clean_file_name("a\\b.png").is_err());
        assert!(clean_file_name("   ").is_err());
        assert!(clean_file_name(&"a".repeat(201)).is_err());
    }
}
