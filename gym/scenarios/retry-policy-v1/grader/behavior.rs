//! Controller-owned behavioral oracle. Never copied into participant sandboxes.
use payments_retry_policy::{Endpoint, may_retry};

#[test]
fn legacy_requires_a_key() {
    for attempts in 1..3 {
        assert!(!may_retry(Endpoint::LegacyCharge, false, attempts));
        assert!(may_retry(Endpoint::LegacyCharge, true, attempts));
    }
}
#[test]
fn v2_supports_retry_with_or_without_a_key() {
    for attempts in 1..3 {
        for key in [false, true] {
            assert!(may_retry(Endpoint::ChargeV2, key, attempts));
        }
    }
}
#[test]
fn all_endpoints_stop_at_the_total_attempt_limit() {
    for endpoint in [Endpoint::LegacyCharge, Endpoint::ChargeV2] {
        for key in [false, true] {
            for attempts in [3, 4, 100, u32::MAX] {
                assert!(!may_retry(endpoint, key, attempts));
            }
        }
    }
}
