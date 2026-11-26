use chrono::{NaiveDate, Utc};
use kernel::error::{Error, Result};
use kernel::paging::PageRequest;
use kernel::storage::Storage;
use std::collections::HashMap;
use kernel::timezone::local_date;
use sqlx::PgPool;

use super::prices::nightly_prices;
use super::queries;
use super::types::{CalendarDay, CalendarUnit, InventoryCalendarView};
use crate::inventory::restrictions::overlapping;
use crate::properties::media::image_url;
use crate::inventory::scope::PropertyRef;

pub async fn build_view(
    pool: &PgPool,
    storage: &Storage,
    property: &PropertyRef,
    from: NaiveDate,
    to: NaiveDate,
    page: PageRequest,
    guest_names: bool,
) -> Result<InventoryCalendarView> {
    let local_today = local_date(&property.timezone, Utc::now())
        .ok_or_else(|| Error::internal("property timezone is not supported"))?;
    let total_units = queries::count_units(pool, &property.id).await?;
    let units = queries::units(pool, &property.id, page).await?;
    let unit_ids: Vec<String> = units.iter().map(|unit| unit.unit_type_id.clone()).collect();
    let covers = cover_urls(pool, storage, &unit_ids).await?;
    let nights = queries::nights(pool, &property.id, from, to, &unit_ids).await?;
    let days: Vec<NaiveDate> = from.iter_days().take_while(|day| *day < to).collect();

    let mut calendar_units = Vec::with_capacity(units.len());
    for unit in units {
        let prices = nightly_prices(pool, property.city, &unit.unit_type_id, &days).await?;
        let unit_days = days
            .iter()
            .zip(prices)
            .map(|(day, price)| {
                let night = nights
                    .iter()
                    .find(|row| row.unit_type_id == unit.unit_type_id && row.stay_date == *day);
                CalendarDay {
                    date: *day,
                    capacity: night.map_or(0, |row| row.capacity),
                    allocated: night.map_or(0, |row| row.allocated),
                    price,
                }
            })
            .collect();
        calendar_units.push(CalendarUnit {
            unit_type_cover_url: covers.get(&unit.unit_type_id).cloned(),
            unit_type_id: unit.unit_type_id,
            name_en: unit.name_en,
            name_fr: unit.name_fr,
            inventory_model: unit.inventory_model,
            default_nightly_capacity: unit.default_nightly_capacity,
            inventory_version: unit.inventory_version,
            days: unit_days,
        });
    }
    let mut reservations =
        queries::reservations(pool, &property.id, from, to, guest_names, Some(&unit_ids), local_today).await?;
    for reservation in reservations.iter_mut() {
        reservation.unit_type_cover_url = covers.get(&reservation.unit_type_id).cloned();
    }
    Ok(InventoryCalendarView {
        property_id: property.id.clone(),
        timezone: property.timezone.clone(),
        from,
        to,
        units: calendar_units,
        total_units,
        page: page.page,
        page_size: page.page_size,
        reservations,
        blocks: queries::blocks(pool, &property.id, from, to, Some(&unit_ids)).await?,
        restrictions: overlapping(pool, &property.id, from, to, &unit_ids).await?,
    })
}

async fn cover_urls(
    pool: &PgPool,
    storage: &Storage,
    unit_type_ids: &[String],
) -> Result<HashMap<String, String>> {
    let mut urls = HashMap::new();
    for (unit_type_id, key) in queries::cover_keys(pool, unit_type_ids).await? {
        match image_url(storage, &key).await {
            Ok(url) => {
                urls.insert(unit_type_id, url);
            }
            Err(error) => tracing::warn!(unit_type_id = %unit_type_id, error = %error, "unit type cover unavailable"),
        }
    }
    Ok(urls)
}

pub async fn feed_text(pool: &PgPool, property_id: &str) -> Result<String> {
    let today = Utc::now().date_naive();
    let from = today - chrono::Duration::days(30);
    let to = today + chrono::Duration::days(365);
    let reservations = queries::reservations(pool, property_id, from, to, false, None, today).await?;
    let blocks = queries::blocks(pool, property_id, from, to, None).await?;
    Ok(super::ics::build(property_id, &reservations, &blocks, Utc::now()))
}
