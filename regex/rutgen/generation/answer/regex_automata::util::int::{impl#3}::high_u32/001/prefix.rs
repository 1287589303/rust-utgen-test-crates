// Answer 0

#[test]
fn test_high_u32_zero() {
    let value: u64 = 0;
    let result = value.high_u32();
}

#[test]
fn test_high_u32_max_low() {
    let value: u64 = u64::MAX >> 32; // Values just below 2^32
    let result = value.high_u32();
}

#[test]
fn test_high_u32_at_threshold() {
    let value: u64 = 1 << 32; // Value equal to 2^32
    let result = value.high_u32();
}

#[test]
fn test_high_u32_max() {
    let value: u64 = u64::MAX; // Value equal to 2^64 - 1
    let result = value.high_u32();
}

