use chrono::NaiveDate;
use kernel::error::Result;
use sqlx::{Postgres, Transaction};

type RuleRow = (Option<i32>, bool, Option<NaiveDate>, Option<NaiveDate>);

pub async fn turnover_blocked(
    tx: &mut Transaction<'_, Postgres>,
    unit_id: &str,
    reservation_id: &str,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Option<i32>> {
    let unit: Option<(String, String)> = sqlx::query_as(
        r#"SELECT "propertyId", "unitTypeId" FROM stay_physical_units WHERE id = $1"#,
    )
    .bind(unit_id)
    .fetch_optional(&mut **tx)
    .await?;
    let Some((property_id, unit_type_id)) = unit else {
        return Ok(None);
    };
    let rules: Vec<RuleRow> = sqlx::query_as(
        r#"SELECT value, ("unitTypeId" IS NOT NULL) AS unit_scoped, "startDate", "endDate"
           FROM stay_restrictions
           WHERE "propertyId" = $1 AND kind = 'TURNOVER_BUFFER_NIGHTS' AND "removedAt" IS NULL
             AND ("unitTypeId" IS NULL OR "unitTypeId" = $2)"#,
    )
    .bind(&property_id)
    .bind(&unit_type_id)
    .fetch_all(&mut **tx)
    .await?;
    let Some(buffer) = chosen_buffer(rules, start, end) else {
        return Ok(None);
    };
    let busy: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_unit_assignments a
               WHERE a."physicalUnitId" = $1 AND a."releasedAt" IS NULL
                 AND a."reservationItemId" NOT IN (SELECT id FROM stay_reservation_items WHERE "reservationId" = $2)
                 AND a."startDate" < $4 + make_interval(days => $5)
                 AND a."endDate" > $3 - make_interval(days => $5))"#,
    )
    .bind(unit_id)
    .bind(reservation_id)
    .bind(start)
    .bind(end)
    .bind(buffer)
    .fetch_one(&mut **tx)
    .await?;
    Ok(if busy { Some(buffer) } else { None })
}

fn chosen_buffer(rules: Vec<RuleRow>, start: NaiveDate, end: NaiveDate) -> Option<i32> {
    let applicable: Vec<(Option<i32>, bool)> = rules
        .into_iter()
        .filter(|(_, _, from, to)| from.map_or(true, |day| day < end) && to.map_or(true, |day| day > start))
        .map(|(value, scoped, _, _)| (value, scoped))
        .collect();
    let specific: Vec<i32> = applicable.iter().filter(|(_, scoped)| *scoped).filter_map(|(value, _)| *value).collect();
    let general: Vec<i32> = applicable.iter().filter(|(_, scoped)| !*scoped).filter_map(|(value, _)| *value).collect();
    let chosen = if specific.is_empty() { general } else { specific };
    chosen.into_iter().max().filter(|buffer| *buffer > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, d).unwrap()
    }

    #[test]
    fn unit_scoped_buffer_overrides_property_buffer() {
        let rules = vec![(Some(3), false, None, None), (Some(1), true, None, None)];
        assert_eq!(chosen_buffer(rules, day(10), day(12)), Some(1));
    }

    #[test]
    fn zero_or_outside_window_means_no_buffer() {
        assert_eq!(chosen_buffer(vec![(Some(0), false, None, None)], day(10), day(12)), None);
        assert_eq!(chosen_buffer(vec![(Some(2), false, Some(day(20)), None)], day(10), day(12)), None);
    }

    #[test]
    fn largest_buffer_wins_inside_a_level() {
        let rules = vec![(Some(1), false, None, None), (Some(2), false, None, None)];
        assert_eq!(chosen_buffer(rules, day(10), day(12)), Some(2));
    }
}
