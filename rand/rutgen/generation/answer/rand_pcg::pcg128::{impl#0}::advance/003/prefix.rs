// Answer 0

#[test]
fn test_advance_with_zero_delta() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.advance(0);
}

#[test]
fn test_advance_with_zero_delta_minimum_state() {
    let mut rng = Lcg128Xsl64::new(0, 67890);
    rng.advance(0);
}

#[test]
fn test_advance_with_zero_delta_maximum_state() {
    let mut rng = Lcg128Xsl64::new(u128::MAX, 67890);
    rng.advance(0);
}

#[test]
fn test_advance_with_zero_delta_minimum_increment() {
    let mut rng = Lcg128Xsl64::new(12345, 0);
    rng.advance(0);
}

#[test]
fn test_advance_with_zero_delta_maximum_increment() {
    let mut rng = Lcg128Xsl64::new(12345, u128::MAX);
    rng.advance(0);
}

