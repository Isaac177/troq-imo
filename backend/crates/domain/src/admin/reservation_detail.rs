use kernel::auth::Actor;
use kernel::error::codes::NOT_FOUND;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, PageRequest, Result};

use super::reservation_detail_media;
use super::reservation_detail_queries as queries;
use super::reservation_detail_types::{Detail, DetailPage};

fn payment_state(state: &str, reference: Option<&str>) -> &'static str {
    if state == "PENDING_PAYMENT" { "AWAITING" } else if reference.is_some() { "PAID" } else { "AT_PROPERTY" }
}

pub async fn get(kernel: &Kernel, actor: &Actor, id: &str, query: DetailPage) -> Result<Detail> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let defaults = PageRequest::default();
    let page = PageRequest { page: query.page.unwrap_or(defaults.page), page_size: query.page_size.unwrap_or(defaults.page_size) }.validated()?;
    let pool = kernel.db.pool();
    let city = actor.city.code();
    let core = queries::core(pool, city, id).await?.ok_or_else(|| Error::not_found(NOT_FOUND))?;
    let payment_state = payment_state(&core.state, core.payment_ref.as_deref()).to_string();
    let cancellation_deadline = core.contract.get("cancellationDeadline").and_then(|value| value.as_str()).and_then(|value| value.parse().ok());
    let mut items = queries::items(pool, id, city, page).await?;
    for item in &mut items.items {
        item.unit_type_cover_url = crate::properties::media::cover_for_unit_type(pool, &kernel.storage, actor.city, &item.unit_type_id).await?;
    }
    Ok(Detail {
        core, payment_state, cancellation_deadline, items,
        unit_type: reservation_detail_media::unit_type(kernel, actor.city, id, page).await?,
        guests: queries::guests(pool, id, city, page).await?,
        timeline: queries::timeline(pool, id, city, page).await?,
        amendments: queries::amendments(pool, id, city, page).await?,
        cases: queries::cases(pool, id, city, page).await?,
    })
}

#[cfg(test)]
mod tests {
    use super::payment_state;

    #[test]
    fn pending_payment_takes_precedence_over_recorded_reference() {
        assert_eq!(payment_state("PENDING_PAYMENT", Some("payment")), "AWAITING");
        assert_eq!(payment_state("CONFIRMED", Some("payment")), "PAID");
        assert_eq!(payment_state("CONFIRMED", None), "AT_PROPERTY");
    }
}
