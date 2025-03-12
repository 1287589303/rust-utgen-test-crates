// Answer 0

#[test]
fn test_advance_positive_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(1);
}

#[test]
fn test_advance_large_odd_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(1_000_000_000_000_000_000_000_000);
}

#[test]
fn test_advance_middle_value_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(2_000_000_000_000_000_000);
}

#[test]
fn test_advance_zero_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    rng.advance(0);
}

