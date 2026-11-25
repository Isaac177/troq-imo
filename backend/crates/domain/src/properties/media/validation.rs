pub const MAX_MEDIA_BYTES: usize = 5 * 1024 * 1024;
pub const ALT_TEXT_LIMIT: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Png,
    Jpeg,
}

impl ImageType {
    pub fn mime(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageRejection {
    Empty,
    TooLarge,
    UnsupportedType,
    ContentMismatch,
    AltTextTooLong,
}

pub fn check_image(mime_type: &str, content: &[u8]) -> Result<ImageType, ImageRejection> {
    if content.is_empty() {
        return Err(ImageRejection::Empty);
    }
    if content.len() > MAX_MEDIA_BYTES {
        return Err(ImageRejection::TooLarge);
    }
    let (image_type, matches) = match mime_type {
        "image/png" => (ImageType::Png, content.starts_with(b"\x89PNG\r\n\x1a\n")),
        "image/jpeg" => (ImageType::Jpeg, content.starts_with(&[0xff, 0xd8, 0xff])),
        _ => return Err(ImageRejection::UnsupportedType),
    };
    if matches { Ok(image_type) } else { Err(ImageRejection::ContentMismatch) }
}

pub fn clean_alt_text(value: &str) -> Result<String, ImageRejection> {
    let trimmed = value.trim();
    if trimmed.chars().count() > ALT_TEXT_LIMIT {
        Err(ImageRejection::AltTextTooLong)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_supported_image_signatures() {
        assert_eq!(check_image("image/png", b"\x89PNG\r\n\x1a\nrest"), Ok(ImageType::Png));
        assert_eq!(check_image("image/jpeg", &[0xff, 0xd8, 0xff, 0xe0]), Ok(ImageType::Jpeg));
        assert_eq!(check_image("image/png", b"%PDF-1.7"), Err(ImageRejection::ContentMismatch));
        assert_eq!(check_image("application/pdf", b"%PDF-1.7"), Err(ImageRejection::UnsupportedType));
    }

    #[test]
    fn rejects_empty_and_oversized_images() {
        assert_eq!(check_image("image/png", b""), Err(ImageRejection::Empty));
        let mut image = b"\x89PNG\r\n\x1a\n".to_vec();
        image.resize(MAX_MEDIA_BYTES + 1, 0);
        assert_eq!(check_image("image/png", &image), Err(ImageRejection::TooLarge));
    }

    #[test]
    fn alt_text_is_trimmed_and_bounded_by_characters() {
        assert_eq!(clean_alt_text("  Lake view  ").unwrap(), "Lake view");
        assert!(clean_alt_text(&"é".repeat(ALT_TEXT_LIMIT)).is_ok());
        assert_eq!(clean_alt_text(&"x".repeat(ALT_TEXT_LIMIT + 1)), Err(ImageRejection::AltTextTooLong));
    }
}
