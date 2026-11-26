use kernel::paging::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::guests::collaborators::MAX_STAY_NIGHTS;
use super::query::{
    ACCESSIBILITY_CODES, CANCELLATION_CLASSES, ENGINE_VERSION, MAX_ADULTS, MAX_CHILDREN,
    MAX_CHILD_AGE, MAX_DISTANCE_KM, MAX_FREE_CANCELLATION_HOURS, MAX_SEARCH_CHARS, MEAL_PLANS,
    PAYMENT_COLLECTIONS, PROPERTY_TYPES, SORTS,
};
use kernel::error::Result;

pub async fn amenity_codes(pool: &PgPool) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(
        r#"SELECT a.code FROM property_amenities a
           JOIN property_amenity_groups g ON g.id = a.group_id
           WHERE a.active AND g.active ORDER BY a.code"#,
    )
    .fetch_all(pool)
    .await?)
}

pub async fn category_codes(pool: &PgPool) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar("SELECT code FROM stay_accommodation_categories WHERE active ORDER BY code")
        .fetch_all(pool)
        .await?)
}

pub fn catalog(amenity_codes: &[String], category_codes: &[String]) -> Value {
    json!({
        "engineVersion": ENGINE_VERSION,
        "endpoint": "/api/public/properties",
        "filters": [
            { "name": "search", "type": "text", "max": MAX_SEARCH_CHARS },
            { "name": "propertyId", "type": "id" },
            { "name": "checkIn", "type": "date", "requires": "checkOut" },
            { "name": "checkOut", "type": "date", "requires": "checkIn" },
            { "name": "adults", "type": "integer", "min": 1, "max": MAX_ADULTS, "default": 1 },
            { "name": "children", "type": "integer", "min": 0, "max": MAX_CHILDREN, "default": 0 },
            { "name": "childAges", "type": "integer-list", "min": 0, "max": MAX_CHILD_AGE, "requires": "one age per child" },
            { "name": "instantBooking", "type": "boolean" },
            { "name": "propertyTypes", "type": "enum-list", "values": PROPERTY_TYPES },
            { "name": "categories", "type": "code-list", "values": category_codes },
            { "name": "mealPlans", "type": "enum-list", "values": MEAL_PLANS },
            { "name": "paymentCollections", "type": "enum-list", "values": PAYMENT_COLLECTIONS },
            { "name": "accessibility", "type": "code-list", "values": ACCESSIBILITY_CODES },
            { "name": "amenities", "type": "code-list", "values": amenity_codes },
            { "name": "priceMin", "type": "money-cdf", "requires": "checkIn and checkOut" },
            { "name": "priceMax", "type": "money-cdf", "requires": "checkIn and checkOut" },
            { "name": "minFreeCancellationHours", "type": "integer", "min": 0, "max": MAX_FREE_CANCELLATION_HOURS },
            { "name": "maxPenaltyPercent", "type": "integer", "min": 0, "max": 100 },
            { "name": "latitude", "type": "number", "requires": "longitude" },
            { "name": "longitude", "type": "number", "requires": "latitude" },
            { "name": "maxDistanceKm", "type": "number", "min_exclusive": 0, "max": MAX_DISTANCE_KM, "requires": "latitude and longitude" },
        ],
        "sorts": [
            { "value": "relevance" },
            { "value": "name" },
            { "value": "price", "requires": "checkIn and checkOut" },
            { "value": "distance", "requires": "latitude and longitude" },
        ],
        "sortValues": SORTS,
        "cancellationClasses": {
            "values": CANCELLATION_CLASSES,
            "rules": {
                "NON_REFUNDABLE": "penalty of 100 percent or more",
                "FLEXIBLE": "no penalty and at least 48 hours free",
                "MODERATE": "everything else",
            },
        },
        "paging": { "page": "from 1", "pageSize": { "min": 1, "max": MAX_PAGE_SIZE, "default": DEFAULT_PAGE_SIZE } },
        "limits": { "maxStayNights": MAX_STAY_NIGHTS },
        "analytics": {
            "parameter": "analyticsConsent",
            "retentionMonths": 6,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_lists_the_code_sets_it_is_given() {
        let value = catalog(&["WIFI".into()], &["VILLA".into()]);
        assert_eq!(value["engineVersion"], ENGINE_VERSION);
        let filters = value["filters"].as_array().unwrap();
        let amenities = filters.iter().find(|filter| filter["name"] == "amenities").unwrap();
        assert_eq!(amenities["values"][0], "WIFI");
        assert_eq!(value["paging"]["pageSize"]["max"], MAX_PAGE_SIZE);
    }
}
