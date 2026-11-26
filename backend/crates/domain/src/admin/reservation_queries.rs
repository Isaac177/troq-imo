use sqlx::{Postgres, QueryBuilder};

use super::reservation_filters::Filters;
use super::reservation_types::ReservationFilter;

pub const FROM: &str = r#" FROM stay_reservations r
    JOIN hosts h ON h.id = r."operatorId" AND h.city_id = r.city_id
    JOIN stay_properties p ON p.id = r."propertyId" AND p.city_id = r.city_id WHERE TRUE"#;

pub fn filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    city: &'a str,
    input: &'a ReservationFilter,
    parsed: &'a Filters<'a>,
    include_state: bool,
) {
    builder.push(" AND r.city_id = ").push_bind(city);
    if include_state {
        if let Some(state) = parsed.state {
            builder.push(" AND r.state = ").push_bind(state);
        }
    }
    if let Some(host) = parsed.host {
        builder.push(r#" AND r."operatorId" = "#).push_bind(host);
    }
    if let Some(property) = parsed.property {
        builder.push(r#" AND r."propertyId" = "#).push_bind(property);
    }
    if input.check_in_from.is_some() || input.check_in_to.is_some() {
        builder.push(r#" AND EXISTS (SELECT 1 FROM stay_reservation_items i WHERE i."reservationId" = r.id"#);
        if let Some(from) = input.check_in_from {
            builder.push(r#" AND i."checkIn" >= "#).push_bind(from);
        }
        if let Some(to) = input.check_in_to {
            builder.push(r#" AND i."checkIn" <= "#).push_bind(to);
        }
        builder.push(")");
    }
    if let Some(search) = &parsed.search {
        builder.push(" AND (r.reference ILIKE ").push_bind(search.clone())
            .push(r#" ESCAPE '\' OR EXISTS (SELECT 1 FROM stay_reservation_items i JOIN stay_reservation_guests g ON g."reservationItemId" = i.id WHERE i."reservationId" = r.id AND (g."fullName" ILIKE "#)
            .push_bind(search.clone()).push(r#" ESCAPE '\' OR g."contactEmail" ILIKE "#)
            .push_bind(search.clone()).push(r#" ESCAPE '\' OR g."contactPhone" ILIKE "#)
            .push_bind(search.clone()).push(r#" ESCAPE '\')))"#);
    }
}

pub const SELECT: &str = r#"SELECT r.id, r.reference, r.state, r."bookingMode" AS booking_mode, r.source,
    r.city_id AS city, r."operatorId" AS operator_id, h.display_name AS operator_name,
    r."propertyId" AS property_id, p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
    (SELECT i."unitTypeId" FROM stay_reservation_items i WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS unit_type_id,
    (SELECT t."nameEn" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId" WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS unit_type_name_en,
    (SELECT t."nameFr" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId" WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS unit_type_name_fr,
    (SELECT g."fullName" FROM stay_reservation_items i JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY' WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS guest_name,
    (SELECT MIN(i."checkIn") FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS check_in,
    (SELECT MAX(i."checkOut") FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS check_out,
    (SELECT (MAX(i."checkOut") - MIN(i."checkIn"))::int FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS nights,
    r."totalAmount"::text AS total_amount, r."commissionAmount"::text AS commission_amount,
    r."operatorNetAmount"::text AS operator_net_amount,
    (SELECT COUNT(*) FROM stay_cases c WHERE c."reservationId" = r.id AND c.state IN ('OPEN', 'AWAITING_RESPONSE')) AS open_cases,
    r."createdAt" AS created_at"#;

pub const SUMMARY: &str = r#"SELECT COUNT(*) AS total,
    COALESCE(SUM(r."totalAmount") FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')), 0)::text AS revenue,
    COUNT(*) FILTER (WHERE r.state = 'REQUESTED') AS requested,
    COUNT(*) FILTER (WHERE r.state = 'PENDING_PAYMENT') AS awaiting_payment,
    COUNT(*) FILTER (WHERE r.state = 'CONFIRMED') AS confirmed,
    COUNT(*) FILTER (WHERE r.state = 'CHECKED_IN') AS checked_in,
    COUNT(*) FILTER (WHERE r.state = 'CANCELLED') AS cancelled,
    COUNT(*) FILTER (WHERE r.state = 'NO_SHOW') AS no_show"#;
