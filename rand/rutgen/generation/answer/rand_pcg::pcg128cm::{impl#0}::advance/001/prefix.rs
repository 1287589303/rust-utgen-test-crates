// Answer 0

#[test]
fn test_advance_mdelta_greater_than_zero_odd() {
    let mut rng = Lcg128CmDxsm64::new(1, 1);
    rng.advance(1); // delta = 1, odd
}

#[test]
fn test_advance_mdelta_greater_than_zero_even() {
    let mut rng = Lcg128CmDxsm64::new(2, 2);
    rng.advance(2); // delta = 2, even
}

#[test]
fn test_advance_mdelta_greater_than_zero_max() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX - 1, 3);
    rng.advance(u128::MAX); // delta = maximum valid u128
}

#[test]
fn test_advance_mdelta_eq_zero() {
    let mut rng = Lcg128CmDxsm64::new(0, 4);
    rng.advance(0); // boundary case, delta = 0
}

