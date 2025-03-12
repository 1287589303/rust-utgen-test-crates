// Answer 0

#[test]
fn test_low_u32_zero() {
    let value: u64 = 0;
    let result = value.low_u32();
}

#[test]
fn test_low_u32_max() {
    let value: u64 = u32::MAX as u64;
    let result = value.low_u32();
}

#[test]
fn test_low_u32_above_u32_max() {
    let value: u64 = (u32::MAX as u64) + 1;
    let result = value.low_u32();
}

#[test]
fn test_low_u32_mid_value() {
    let value: u64 = 123456789;
    let result = value.low_u32();
}

#[test]
fn test_low_u32_large_value() {
    let value: u64 = 2000000000;
    let result = value.low_u32();
}

