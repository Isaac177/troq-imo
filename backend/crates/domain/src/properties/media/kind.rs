use super::storage::ImageFolder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Property,
    Template,
}

impl MediaKind {
    pub fn table(self) -> &'static str {
        match self {
            Self::Property => "stay_property_media",
            Self::Template => "stay_template_media",
        }
    }

    pub fn owner_column(self) -> &'static str {
        match self {
            Self::Property => r#""propertyId""#,
            Self::Template => r#""templateId""#,
        }
    }

    pub fn owner_table(self) -> &'static str {
        match self {
            Self::Property => "stay_properties",
            Self::Template => "stay_accommodation_templates",
        }
    }

    pub fn folder(self) -> ImageFolder {
        match self {
            Self::Property => ImageFolder::Property,
            Self::Template => ImageFolder::Template,
        }
    }

    pub fn resource_type(self) -> &'static str {
        match self {
            Self::Property => "property",
            Self::Template => "accommodation_template",
        }
    }
}
