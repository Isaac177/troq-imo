use super::validators::{clean_name, clean_visibility, parse_property_ids};

#[test]
fn names_are_trimmed_and_limited() {
    assert_eq!(clean_name("  Lake trip ").unwrap(), "Lake trip");
    assert!(clean_name("   ").is_err());
    assert!(clean_name(&"x".repeat(81)).is_err());
    assert!(clean_name(&"x".repeat(80)).is_ok());
}

#[test]
fn visibility_defaults_to_private_and_refuses_unknown_values() {
    assert_eq!(clean_visibility(None).unwrap(), "PRIVATE");
    assert_eq!(clean_visibility(Some("SHARED")).unwrap(), "SHARED");
    assert!(clean_visibility(Some("PUBLIC")).is_err());
}

#[test]
fn property_filter_is_a_bounded_comma_list() {
    assert_eq!(parse_property_ids(" a, b ,,c").unwrap(), vec!["a", "b", "c"]);
    assert!(parse_property_ids(" , ").is_err());
    let many = (0..101).map(|n| format!("p{n}")).collect::<Vec<_>>().join(",");
    assert!(parse_property_ids(&many).is_err());
    assert!(parse_property_ids(&"x".repeat(129)).is_err());
}
