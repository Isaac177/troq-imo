use axum::async_trait;
use domain::billing::{self, commission, credits};
use domain::inventory::restrictions::{self, StayRequest};
use domain::pricing::quotes::{CommissionQuote, QuoteChecks, StayWindow};
use domain::properties::media;
use kernel::{City, Kernel, Money, Result};

pub struct DomainQuoteChecks {
    kernel: Kernel,
}

impl DomainQuoteChecks {
    pub fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }
}

#[async_trait]
impl QuoteChecks for DomainQuoteChecks {
    async fn enforce_restrictions(&self, city: City, property_id: &str, window: &StayWindow) -> Result<()> {
        let request = StayRequest {
            unit_type_id: window.unit_type_id.clone(),
            check_in: window.check_in,
            check_out: window.check_out,
            booking_day: window.booking_day,
        };
        restrictions::enforce_for_city(self.kernel.db.pool(), city, property_id, &request).await
    }

    async fn commission(&self, city: City, host_id: &str, total: Money) -> Result<CommissionQuote> {
        let pool = self.kernel.db.pool();
        billing::ensure_host_in_city(pool, host_id, city).await?;
        let result = commission::rate::snapshot_for(pool, city, host_id, total).await?;
        Ok(CommissionQuote { rate_bps: result.rate_bps, amount: result.amount, net: result.net })
    }

    async fn ensure_can_book(&self, city: City, host_id: &str, commission: Money) -> Result<()> {
        let pool = self.kernel.db.pool();
        billing::ensure_host_in_city(pool, host_id, city).await?;
        credits::gate::ensure_can_book(pool, host_id, commission.minor()).await
    }

    async fn cover_url(&self, city: City, unit_type_id: &str) -> Result<Option<String>> {
        media::cover_for_unit_type(self.kernel.db.pool(), &self.kernel.storage, city, unit_type_id).await
    }
}
