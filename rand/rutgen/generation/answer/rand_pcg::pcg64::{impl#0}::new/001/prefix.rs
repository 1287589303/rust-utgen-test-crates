// Answer 0

#[test]
fn test_new_with_zero_state_and_zero_stream() {
    let rng = Lcg64Xsh32::new(0x0, 0x0);
}

#[test]
fn test_new_with_max_state_and_zero_stream() {
    let rng = Lcg64Xsh32::new(0xffffffffffffffff, 0x0);
}

#[test]
fn test_new_with_zero_state_and_max_stream() {
    let rng = Lcg64Xsh32::new(0x0, 0x7ffffffffffffffe);
}

#[test]
fn test_new_with_mid_state_and_mid_stream() {
    let rng = Lcg64Xsh32::new(0x7fffffffffffffff, 0x3fffffffffffffff);
}

#[test]
fn test_new_with_non_zero_state_and_non_zero_stream() {
    let rng = Lcg64Xsh32::new(0x1234567890abcdef, 0x1ffffffffffffffe);
}

#[test]
fn test_new_with_incremented_stream() {
    let rng = Lcg64Xsh32::new(0xfedcba9876543210, 0x1);
}

