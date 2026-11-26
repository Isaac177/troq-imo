use axum::async_trait;
use chrono::NaiveDate;
use kernel::{City, Money, Result};

use super::types::CommissionQuote;

#[derive(Debug, Clone)]
pub struct StayWindow {
    pub unit_type_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub booking_day: NaiveDate,
}

#[async_trait]
pub trait QuoteChecks: Send + Sync {
    async fn enforce_restrictions(&self, city: City, property_id: &str, window: &StayWindow) -> Result<()>;

    async fn commission(&self, city: City, host_id: &str, total: Money) -> Result<CommissionQuote>;

    async fn ensure_can_book(&self, city: City, host_id: &str, commission: Money) -> Result<()>;

    async fn cover_url(&self, city: City, unit_type_id: &str) -> Result<Option<String>>;
}
