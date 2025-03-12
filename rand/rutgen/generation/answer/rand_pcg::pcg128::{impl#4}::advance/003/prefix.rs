// Answer 0

#[test]
fn test_advance_delta_zero() {
    let mut rng = Mcg128Xsl64::new(1);
    rng.advance(0);
}

#[test]
fn test_advance_delta_one() {
    let mut rng = Mcg128Xsl64::new(1);
    rng.advance(1);
}

#[test]
fn test_advance_delta_two_pow_63_minus_1() {
    let mut rng = Mcg128Xsl64::new(1);
    rng.advance(2u128.pow(63) - 1);
}

#[test]
fn test_advance_delta_two_pow_64() {
    let mut rng = Mcg128Xsl64::new(1);
    rng.advance(2u128.pow(64));
}

#[test]
fn test_advance_delta_two_pow_128_minus_1() {
    let mut rng = Mcg128Xsl64::new(1);
    rng.advance(2u128.pow(128) - 1);
}

