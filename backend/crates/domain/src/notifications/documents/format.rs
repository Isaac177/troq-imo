use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime};

pub fn cdf(amount: f64) -> String {
    let whole = format!("{:.0}", amount.abs().round());
    let mut grouped = String::new();
    for (index, ch) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index) % 3 == 0 {
            grouped.push('\u{00a0}');
        }
        grouped.push(ch);
    }
    let sign = if amount < 0.0 && whole != "0" { "-" } else { "" };
    format!("{sign}{grouped}\u{00a0}CDF")
}

pub fn date_ddmmyyyy(raw: &str) -> Result<String> {
    let value = raw.trim();
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Ok(parsed.format("%d/%m/%Y").to_string());
    }
    if let Ok(parsed) = NaiveDateTime::parse_from_str(value.trim_end_matches(" UTC"), "%Y-%m-%d %H:%M") {
        return Ok(parsed.format("%d/%m/%Y").to_string());
    }
    if let Ok(parsed) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Ok(parsed.format("%d/%m/%Y").to_string());
    }
    Err(anyhow!("unsupported date value: {value}"))
}

pub fn require_cdf(currency: &str) -> Result<()> {
    if currency.trim().eq_ignore_ascii_case("CDF") {
        Ok(())
    } else {
        bail!("documents are CDF only, got currency {currency}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_thousands_with_a_non_breaking_space() {
        assert_eq!(cdf(450000.0), "450\u{00a0}000\u{00a0}CDF");
        assert_eq!(cdf(0.0), "0\u{00a0}CDF");
        assert_eq!(cdf(-1500.0), "-1\u{00a0}500\u{00a0}CDF");
    }

    #[test]
    fn dates_display_as_ddmmyyyy() {
        assert_eq!(date_ddmmyyyy("2026-11-02").unwrap(), "02/11/2026");
        assert_eq!(date_ddmmyyyy("2026-11-12T10:00:00+00:00").unwrap(), "12/11/2026");
        assert_eq!(date_ddmmyyyy("2026-11-12 10:00 UTC").unwrap(), "12/11/2026");
        assert!(date_ddmmyyyy("12/11/2026").is_err());
    }

    #[test]
    fn only_cdf_is_accepted() {
        assert!(require_cdf(" cdf ").is_ok());
        assert!(require_cdf("USD").is_err());
    }
}
