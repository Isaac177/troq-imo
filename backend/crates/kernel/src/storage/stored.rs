#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoredRef {
    Empty,
    Key(String),
    External(String),
    Inline(String),
}

pub fn classify(value: &str) -> StoredRef {
    let value = value.trim();
    if value.is_empty() {
        StoredRef::Empty
    } else if value.starts_with("data:") || value.starts_with("blob:") {
        StoredRef::Inline(value.to_string())
    } else if value.starts_with("http://") || value.starts_with("https://") {
        StoredRef::External(value.to_string())
    } else {
        StoredRef::Key(value.trim_start_matches('/').to_string())
    }
}

pub fn public_url(base: &str, key: &str) -> String {
    format!("{}/{}", base.trim_end_matches('/'), key.trim_start_matches('/'))
}

pub fn resolve_public(base: &str, value: &str) -> String {
    match classify(value) {
        StoredRef::Empty => String::new(),
        StoredRef::Inline(raw) | StoredRef::External(raw) => raw,
        StoredRef::Key(key) => public_url(base, &key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://media.mungini.example";

    #[test]
    fn resolves_every_shape() {
        assert_eq!(resolve_public(BASE, ""), "");
        assert_eq!(
            resolve_public(BASE, "global/brands/a.png"),
            "https://media.mungini.example/global/brands/a.png"
        );
        assert_eq!(resolve_public(BASE, "https://cdn.partner.com/l.png"), "https://cdn.partner.com/l.png");
        assert_eq!(resolve_public(BASE, "data:image/png;base64,AA"), "data:image/png;base64,AA");
    }
}
