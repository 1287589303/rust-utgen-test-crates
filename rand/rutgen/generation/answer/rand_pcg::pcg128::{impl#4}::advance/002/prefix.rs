// Answer 0

#[test]
fn test_advance_positive_mdelta() {
    let mut rng = Mcg128Xsl64::new(12345);
    rng.advance(5);
}

#[test]
fn test_advance_even_mdelta() {
    let mut rng = Mcg128Xsl64::new(67890);
    rng.advance(4);
}

#[test]
fn test_advance_zero_mdelta() {
    let mut rng = Mcg128Xsl64::new(13579);
    rng.advance(0);
}

