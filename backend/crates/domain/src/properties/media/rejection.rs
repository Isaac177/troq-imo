use super::validation::ImageRejection;

impl ImageRejection {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "MEDIA_EMPTY",
            Self::TooLarge => "MEDIA_TOO_LARGE",
            Self::UnsupportedType => "MEDIA_UNSUPPORTED_TYPE",
            Self::ContentMismatch => "MEDIA_CONTENT_MISMATCH",
            Self::AltTextTooLong => "MEDIA_ALT_TEXT_TOO_LONG",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rejection_has_a_stable_code() {
        assert_eq!(ImageRejection::Empty.code(), "MEDIA_EMPTY");
        assert_eq!(ImageRejection::ContentMismatch.code(), "MEDIA_CONTENT_MISMATCH");
    }
}
