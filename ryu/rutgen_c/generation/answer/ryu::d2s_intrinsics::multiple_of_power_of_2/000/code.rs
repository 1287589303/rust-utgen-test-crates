// Answer 0

#[test]
fn test_multiple_of_power_of_2_non_zero() {
    assert!(multiple_of_power_of_2(8, 3)); // 8 is multiple of 2^3
    assert!(!multiple_of_power_of_2(10, 3)); // 10 is not a multiple of 2^3
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    let _ = multiple_of_power_of_2(0, 3); // should panic due to zero value
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_high_p() {
    let _ = multiple_of_power_of_2(8, 64); // should panic due to p >= 64
}

#[test]
fn test_multiple_of_power_of_2_edge_cases() {
    assert!(multiple_of_power_of_2(1 << 0, 1)); // 1 is a multiple of 2^1
    assert!(multiple_of_power_of_2(1 << 1, 1)); // 2 is a multiple of 2^1
    assert!(multiple_of_power_of_2(1 << 5, 5)); // 32 is a multiple of 2^5
    assert!(!multiple_of_power_of_2(1 << 6, 5)); // 64 is not a multiple of 2^5
}

