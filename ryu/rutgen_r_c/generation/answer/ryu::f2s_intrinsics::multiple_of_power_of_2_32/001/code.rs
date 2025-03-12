// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_with_zero() {
    let value: u32 = 0;
    let p: u32 = 5;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_32_with_non_multiple() {
    let value: u32 = 10; // 1010 in binary
    let p: u32 = 3;
    assert_eq!(multiple_of_power_of_2_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_32_with_multiple() {
    let value: u32 = 8; // 1000 in binary
    let p: u32 = 3;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_32_with_p_zero() {
    let value: u32 = 1; // any value with p = 0 results in true if value is not zero
    let p: u32 = 0;
    assert_eq!(multiple_of_power_of_2_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_32_with_large_value() {
    let value: u32 = 0xFFFFFFFF; // All bits set
    let p: u32 = 32; // 32 bits, should be true
    assert_eq!(multiple_of_power_of_2_32(value, p), false);
}

