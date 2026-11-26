#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    En,
    Fr,
}

impl Locale {
    pub fn parse(value: Option<&str>) -> Self {
        match value.map(|v| v.trim().to_ascii_lowercase()) {
            Some(v) if v.starts_with("en") => Self::En,
            _ => Self::Fr,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentKind {
    StayVoucher,
    StayCancellation,
    StayDamageClaim,
}

impl DocumentKind {
    pub const ALL: [DocumentKind; 3] = [Self::StayVoucher, Self::StayCancellation, Self::StayDamageClaim];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::StayVoucher => "stay_voucher",
            Self::StayCancellation => "stay_cancellation",
            Self::StayDamageClaim => "stay_damage_claim",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == value)
    }

    pub fn template_path(self) -> &'static str {
        match self {
            Self::StayVoucher => "templates/documents/stay_voucher.hbs",
            Self::StayCancellation => "templates/documents/stay_cancellation.hbs",
            Self::StayDamageClaim => "templates/documents/stay_damage_claim.hbs",
        }
    }

    pub fn file_prefix(self) -> &'static str {
        match self {
            Self::StayVoucher => "stay-voucher",
            Self::StayCancellation => "stay-cancellation",
            Self::StayDamageClaim => "stay-damage-claim",
        }
    }

    pub fn section(self) -> &'static str {
        match self {
            Self::StayVoucher => "stayVoucher",
            Self::StayCancellation => "stayCancellation",
            Self::StayDamageClaim => "stayDamageClaim",
        }
    }

    pub fn entity_type(self) -> &'static str {
        match self {
            Self::StayDamageClaim => "stay_case",
            _ => "reservation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_defaults_to_french() {
        assert_eq!(Locale::parse(None), Locale::Fr);
        assert_eq!(Locale::parse(Some("EN-GB")), Locale::En);
        assert_eq!(Locale::parse(Some("sw")), Locale::Fr);
    }

    #[test]
    fn kinds_round_trip_and_own_their_entity() {
        for kind in DocumentKind::ALL {
            assert_eq!(DocumentKind::parse(kind.as_str()), Some(kind));
        }
        assert_eq!(DocumentKind::StayDamageClaim.entity_type(), "stay_case");
        assert_eq!(DocumentKind::StayVoucher.entity_type(), "reservation");
        assert_eq!(DocumentKind::parse("invoice"), None);
    }
}
