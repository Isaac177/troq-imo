use std::path::Path;

use anyhow::Result;
use serde_json::Value;

use super::cancellation::CancellationPayload;
use super::damage_claim::DamageClaimPayload;
use super::kind::{DocumentKind, Locale};
use super::voucher::VoucherPayload;

pub enum Payload {
    Voucher(VoucherPayload),
    Cancellation(CancellationPayload),
    DamageClaim(DamageClaimPayload),
}

impl Payload {
    pub fn parse(kind: DocumentKind, snapshot: &Value, locale: Locale) -> Result<Option<Self>> {
        Ok(match kind {
            DocumentKind::StayVoucher => VoucherPayload::from_snapshot(snapshot, locale)?.map(Self::Voucher),
            DocumentKind::StayCancellation => CancellationPayload::from_snapshot(snapshot, locale)?.map(Self::Cancellation),
            DocumentKind::StayDamageClaim => DamageClaimPayload::from_event(snapshot, locale)?.map(Self::DamageClaim),
        })
    }

    pub fn entity_id(&self) -> &str {
        match self {
            Self::Voucher(payload) => &payload.reservation_id,
            Self::Cancellation(payload) => &payload.reservation_id,
            Self::DamageClaim(payload) => &payload.case_id,
        }
    }

    pub fn reference(&self) -> &str {
        match self {
            Self::Voucher(payload) => &payload.reference,
            Self::Cancellation(payload) => &payload.reference,
            Self::DamageClaim(payload) => &payload.reference,
        }
    }

    pub fn required_values(&self) -> Vec<String> {
        match self {
            Self::Voucher(payload) => payload.required_values(),
            Self::Cancellation(payload) => payload.required_values(),
            Self::DamageClaim(payload) => payload.required_values(),
        }
    }

    pub fn view(&self, root: &Path, brand: &str) -> Result<Value> {
        match self {
            Self::Voucher(payload) => payload.view(root, brand),
            Self::Cancellation(payload) => payload.view(root, brand),
            Self::DamageClaim(payload) => payload.view(root, brand),
        }
    }
}
