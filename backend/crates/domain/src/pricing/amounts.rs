use kernel::Money;

pub fn percent_of(base_minor: i64, percent_minor: i64) -> i64 {
    Money::from_minor(base_minor).percent_of(percent_minor).minor()
}

pub fn money(minor: i64) -> Money {
    Money::from_minor(minor)
}

pub fn parse_stored(text: &str) -> kernel::Result<i64> {
    Ok(Money::parse(text)?.minor())
}
