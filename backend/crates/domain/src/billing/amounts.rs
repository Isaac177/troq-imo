use kernel::Money;

pub fn wire(minor: i64) -> String {
    let sign = if minor < 0 { "-" } else { "" };
    format!("{sign}{}", Money::from_minor(minor.abs()).to_wire())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_amounts_keep_their_sign_and_two_decimals() {
        assert_eq!(wire(-1_050), "-10.50");
        assert_eq!(wire(-5), "-0.05");
        assert_eq!(wire(0), "0.00");
        assert_eq!(wire(5_000_000), "50000.00");
    }
}
