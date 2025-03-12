// Answer 0

#[test]
fn test_advance_with_delta_zero_and_zero_state() {
    let mut rng = Lcg128CmDxsm64::new(0, 0);
    rng.advance(0);
}

#[test]
fn test_advance_with_delta_zero_and_max_state() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX, 0);
    rng.advance(0);
}

#[test]
fn test_advance_with_delta_zero_and_min_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, 1);
    rng.advance(0);
}

#[test]
fn test_advance_with_delta_zero_and_max_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, u128::MAX);
    rng.advance(0);
}

#[test]
fn test_advance_with_delta_zero_and_random_state_increment() {
    let mut rng = Lcg128CmDxsm64::new(12345678901234567890, 98765432109876543210);
    rng.advance(0);
}

