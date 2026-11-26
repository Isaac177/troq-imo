pub fn matches(pattern: &str, event_type: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    match pattern.strip_suffix(".*") {
        Some(prefix) => event_type
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('.')),
        None => pattern == event_type,
    }
}

pub fn matches_any(patterns: &[&str], event_type: &str) -> bool {
    patterns.iter().any(|pattern| matches(pattern, event_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_pattern_matches_only_that_type() {
        assert!(matches("reservation.held", "reservation.held"));
        assert!(!matches("reservation.held", "reservation.held_late"));
    }

    #[test]
    fn prefix_pattern_matches_the_family() {
        assert!(matches("reservation.*", "reservation.expired"));
        assert!(matches("reservation.*", "reservation.amendment_accepted"));
        assert!(!matches("reservation.*", "reservationx.expired"));
        assert!(!matches("reservation.*", "reservation"));
        assert!(!matches("case.*", "reservation.case.closed"));
    }

    #[test]
    fn star_matches_everything() {
        assert!(matches("*", "property.created"));
    }

    #[test]
    fn any_pattern_is_enough() {
        assert!(matches_any(&["case.closed", "property.*"], "property.created"));
        assert!(!matches_any(&[], "property.created"));
    }
}
