// Answer 0

#[test]
fn test_new_with_zero_state() {
    let state = 0;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_one_state() {
    let state = 1;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_max_state() {
    let state = u128::MAX;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_default_state() {
    let state = 0xcafef00dd15ea5e5;
    let rng = Mcg128Xsl64::new(state);
}

