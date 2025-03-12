// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_with_zero_value() {
    let value: u32 = 0;
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_32_with_non_zero_value() {
    let value: u32 = 25; // 5^2
    let p: u32 = 2;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_value_not_multiple_of_5() {
    let value: u32 = 7;
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_32_with_high_power() {
    let value: u32 = 125; // 5^3
    let p: u32 = 3;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_exceeding_power() {
    let value: u32 = 125; // 5^3
    let p: u32 = 4;
    assert_eq!(multiple_of_power_of_5_32(value, p), false);
}

