// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_zero() {
    assert_eq!(multiple_of_power_of_5_32(0, 0), true);
}

#[test]
fn test_multiple_of_power_of_5_32_non_zero() {
    assert_eq!(multiple_of_power_of_5_32(5, 1), true);
    assert_eq!(multiple_of_power_of_5_32(25, 2), true);
    assert_eq!(multiple_of_power_of_5_32(125, 3), true);
    assert_eq!(multiple_of_power_of_5_32(10, 1), true);
    assert_eq!(multiple_of_power_of_5_32(30, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_32_high_power() {
    assert_eq!(multiple_of_power_of_5_32(5, 2), false);
    assert_eq!(multiple_of_power_of_5_32(0, 1), false);
}

