//! Starter payment-client retry policy. The scenario asks participants to repair it.
#[derive(Clone, Copy, Debug)]
pub enum Endpoint {
    LegacyCharge,
    ChargeV2,
}

/// `attempts` counts attempts already sent, including the one that just failed.
pub fn may_retry(_endpoint: Endpoint, _has_idempotency_key: bool, attempts: u32) -> bool {
    attempts < 3
}
