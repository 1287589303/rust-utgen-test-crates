// Answer 0

#[test]
fn test_from_state_incr_minimal_increment() {
    let state: u128 = 0;
    let increment: u128 = 1;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_maximal_increment() {
    let state: u128 = 0;
    let increment: u128 = u128::MAX;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_minimal_state() {
    let state: u128 = 1;
    let increment: u128 = 2;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_mid_range() {
    let state: u128 = u128::MAX / 2;
    let increment: u128 = u128::MAX / 2 + 1;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_maximal_state() {
    let state: u128 = u128::MAX;
    let increment: u128 = u128::MAX - 1;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
}

