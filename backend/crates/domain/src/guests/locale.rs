const DEFAULT_LOCALE: &str = "en";
const MAX_LOCALE_CHARS: usize = 16;

pub fn clean_locale(value: Option<&str>) -> String {
    let trimmed = value.map(str::trim).unwrap_or_default().to_ascii_lowercase();
    let valid = !trimmed.is_empty()
        && trimmed.chars().count() <= MAX_LOCALE_CHARS
        && trimmed.chars().all(|character| character.is_ascii_alphanumeric() || character == '-');
    if valid {
        trimmed
    } else {
        DEFAULT_LOCALE.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_defaults_to_english() {
        assert_eq!(clean_locale(None), "en");
        assert_eq!(clean_locale(Some(" FR ")), "fr");
        assert_eq!(clean_locale(Some("fr; DROP")), "en");
    }
}
