// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_true() {
    assert_eq!(multiple_of_power_of_2_32(0, 1), true);
    assert_eq!(multiple_of_power_of_2_32(0, 32), true);
    assert_eq!(multiple_of_power_of_2_32(8, 3), true);
}

#[test]
fn test_multiple_of_power_of_2_32_false() {
    assert_eq!(multiple_of_power_of_2_32(5, 2), false);
    assert_eq!(multiple_of_power_of_2_32(10, 4), false);
}

#[test]
fn test_multiple_of_power_of_2_32_boundary() {
    assert_eq!(multiple_of_power_of_2_32(1, 1), false);
    assert_eq!(multiple_of_power_of_2_32(3, 2), false);
    assert_eq!(multiple_of_power_of_2_32(2, 2), true);
    assert_eq!(multiple_of_power_of_2_32(16, 4), true);
}

