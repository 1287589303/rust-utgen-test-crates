// Answer 0

#[test]
fn test_multiple_of_power_of_2() {
    assert!(multiple_of_power_of_2(8, 3)); // 8 is 2^3, should return true
    assert!(multiple_of_power_of_2(16, 4)); // 16 is 2^4, should return true
    assert!(!multiple_of_power_of_2(12, 3)); // 12 is not a multiple of 2^3, should return false
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    multiple_of_power_of_2(0, 3); // Should panic due to debug_assert!(value != 0)
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_high_power() {
    multiple_of_power_of_2(4, 64); // Should panic due to debug_assert!(p < 64)
}

