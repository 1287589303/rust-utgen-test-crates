// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_true() {
    let value = 8; // 1000 in binary
    let p = 3;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_32_false() {
    let value = 10; // 1010 in binary
    let p = 3;
    assert_eq!(multiple_of_power_of_2_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_32_zero_value() {
    let value = 0;
    let p = 5;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_32_large_power() {
    let value = 1; // 0001 in binary
    let p = 1;
    assert_eq!(multiple_of_power_of_2_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_32_max_value() {
    let value = u32::MAX; // 32 bits set to 1
    let p = 31;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_32_edge_case() {
    let value = 16; // 10000 in binary
    let p = 4;
    assert_eq!(multiple_of_power_of_2_32(value, p), true);
}

