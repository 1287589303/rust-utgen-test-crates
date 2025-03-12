// Answer 0

#[test]
fn test_from_state_incr_zero_state_one_increment() {
    let state = 0;
    let increment = 1;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_max_state_max_increment() {
    let state = u64::MAX;
    let increment = u64::MAX;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_mid_state_mid_increment() {
    let state = u64::MAX / 2;
    let increment = u64::MAX / 2;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_large_state_small_increment() {
    let state = u64::MAX - 1;
    let increment = 1;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_small_state_large_increment() {
    let state = 1;
    let increment = u64::MAX - 1;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_one_state_one_increment() {
    let state = 1;
    let increment = 1;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_zero_state_large_increment() {
    let state = 0;
    let increment = u64::MAX;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

#[test]
fn test_from_state_incr_zero_state_mid_increment() {
    let state = 0;
    let increment = u64::MAX / 2;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
}

