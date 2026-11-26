pub fn host_enabled(host: &str) -> String {
    format!(
        r#"(NOT (COALESCE((SELECT cr."rateBps" FROM stay_operator_commission_rates cr
              WHERE cr."operatorId" = {host}.id AND cr."effectiveFrom" <= now()
              ORDER BY cr."effectiveFrom" DESC, cr."createdAt" DESC, cr.id DESC LIMIT 1),
              COALESCE((SELECT ps."valueInt" FROM stay_platform_settings ps
                         WHERE ps.city_id = {host}.city_id AND ps.key = 'DEFAULT_COMMISSION_BPS'), 0)) > 0
          AND COALESCE((SELECT a.balance FROM stay_credit_accounts a WHERE a."operatorId" = {host}.id), 0)
              <= COALESCE((SELECT ps."valueInt" FROM stay_platform_settings ps
                            WHERE ps.city_id = {host}.city_id AND ps.key = 'MIN_CREDIT_CDF'), 0)))"#
    )
}

pub fn listed(property: &str, host: &str) -> String {
    format!(
        "({property}.status = 'PUBLISHED' AND {host}.status = 'ACTIVE' AND {gate})",
        gate = host_enabled(host)
    )
}

fn pick(alias: &str, english: &str, french: &str, locale: &str) -> String {
    format!(
        r#"(CASE WHEN {locale} = 'fr' THEN COALESCE(NULLIF({alias}."{french}", ''), {alias}."{english}")
             ELSE COALESCE(NULLIF({alias}."{english}", ''), {alias}."{french}") END)"#
    )
}

pub fn property_name(alias: &str, locale: &str) -> String {
    pick(alias, "nameEn", "nameFr", locale)
}

pub fn property_description(alias: &str, locale: &str) -> String {
    pick(alias, "descriptionEn", "descriptionFr", locale)
}

pub fn unit_type_name(alias: &str, locale: &str) -> String {
    pick(alias, "nameEn", "nameFr", locale)
}

pub fn unit_type_description(alias: &str, locale: &str) -> String {
    pick(alias, "descriptionEn", "descriptionFr", locale)
}

pub fn media_alt(alias: &str, locale: &str) -> String {
    format!("{} AS alt", pick(alias, "altEn", "altFr", locale))
}

pub fn template_media_alt(alias: &str, locale: &str) -> String {
    media_alt(alias, locale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listed_requires_published_active_host_and_credit_gate() {
        let sql = listed("p", "h");
        assert!(sql.contains("p.status = 'PUBLISHED'"));
        assert!(sql.contains("h.status = 'ACTIVE'"));
        assert!(sql.contains("stay_credit_accounts"));
        assert!(sql.contains(r#"cr."operatorId" = h.id"#));
        assert!(sql.contains("ps.city_id = h.city_id"));
    }

    #[test]
    fn names_choose_the_requested_language_and_fall_back_to_the_other() {
        let sql = property_name("p", "$3");
        assert!(sql.contains("$3 = 'fr'"));
        assert!(sql.contains(r#"COALESCE(NULLIF(p."nameFr", ''), p."nameEn")"#));
        assert!(sql.contains(r#"COALESCE(NULLIF(p."nameEn", ''), p."nameFr")"#));
    }

    #[test]
    fn alt_text_uses_the_alt_columns() {
        assert!(media_alt("m", "$2").ends_with("AS alt"));
        assert!(media_alt("m", "$2").contains(r#"m."altFr""#));
    }
}
