// Answer 0

#[test]
fn test_new_with_minimum_state_and_stream() {
    let state: u128 = 0;
    let stream: u128 = 0;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_maximum_state_and_stream() {
    let state: u128 = u128::MAX;
    let stream: u128 = u128::MAX;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_mid_range_state_and_stream() {
    let state: u128 = u128::MAX / 2;
    let stream: u128 = u128::MAX / 2;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_highest_bit_masked_stream() {
    let state: u128 = 0x123456789abcdef0;
    let stream: u128 = 0x7fffffffffffffff; // Highest bit is discarded.
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_boundary_stream() {
    let state: u128 = 0xaabbccddeeff0011;
    let stream: u128 = 0x8000000000000000; // Highest bit set.
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_zero_state_and_high_stream() {
    let state: u128 = 0;
    let stream: u128 = u128::MAX - 1; // Still an odd increment after masking.
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_small_odd_stream() {
    let state: u128 = 0x1;
    let stream: u128 = 0x3; // Just above the minimum for the increment to be odd.
    let rng = Lcg128Xsl64::new(state, stream);
}

