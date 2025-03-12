// Answer 0

#[test]
fn test_leading_zeros_zero() {
    let bitmask = BitMask(0);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_max() {
    let bitmask = BitMask(u64::MAX); // Assuming BitMaskWord is a u64
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_mid_range() {
    let bitmask = BitMask(0b00010000); // Example mid-range value
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_another_mid_range() {
    let bitmask = BitMask(0b00100000); // Another example mid-range value
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_all_bits_set_except_one() {
    let bitmask = BitMask(0b01111111); // One zero bit at the start
    let result = bitmask.leading_zeros();
}

