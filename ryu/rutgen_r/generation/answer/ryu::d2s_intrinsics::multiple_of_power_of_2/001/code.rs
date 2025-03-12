// Answer 0

#[test]
fn test_multiple_of_power_of_2_boundary_case() {
    let value: u64 = 4; // 100 in binary, which is a multiple of 2^2
    let p: u32 = 2; // p is 2
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_not_multiple() {
    let value: u64 = 5; // 101 in binary, which is not a multiple of 2^2
    let p: u32 = 2; // p is 2
    assert!(!multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_zero_case() {
    let value: u64 = 8; // 1000 in binary, which is a multiple of 2^3
    let p: u32 = 3; // p is 3
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_with_different_power() {
    let value: u64 = 16; // 10000 in binary, which is a multiple of 2^4
    let p: u32 = 4; // p is 4
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_non_power_of_two() {
    let value: u64 = 12; // 1100 in binary, which is a multiple of 2^2 but not of 2^3
    let p: u32 = 3; // p is 3
    assert!(!multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_large_value() {
    let value: u64 = 1 << 62; // 2^62, which is a multiple of 2^62
    let p: u32 = 62; // p is 62
    assert!(multiple_of_power_of_2(value, p));
}

