// Answer 0

#[test]
fn test_advance_positive_delta_1() {
    let mut rng = Mcg128Xsl64::new(0x1234567890abcdef1234567890abcdef);
    rng.advance(1);
}

#[test]
fn test_advance_positive_delta_3() {
    let mut rng = Mcg128Xsl64::new(0x1234567890abcdef1234567890abcdef);
    rng.advance(3);
}

#[test]
fn test_advance_positive_delta_power_of_two() {
    let mut rng = Mcg128Xsl64::new(0x1234567890abcdef1234567890abcdef);
    rng.advance(16);
}

#[test]
fn test_advance_positive_delta_just_below_power_of_two() {
    let mut rng = Mcg128Xsl64::new(0x1234567890abcdef1234567890abcdef);
    rng.advance(15);
}

#[test]
fn test_advance_delta_zero() {
    let mut rng = Mcg128Xsl64::new(0x1234567890abcdef1234567890abcdef);
    rng.advance(0);
}

