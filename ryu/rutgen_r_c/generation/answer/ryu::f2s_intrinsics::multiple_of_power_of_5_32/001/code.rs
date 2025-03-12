// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_case_1() {
    let value: u32 = 25; // 25 = 5^2
    let p: u32 = 2;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_case_2() {
    let value: u32 = 10; // 10 = 5^1 * 2
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_case_3() {
    let value: u32 = 6; // 6 has no factor of 5
    let p: u32 = 1;
    assert_eq!(multiple_of_power_of_5_32(value, p), false);
}

#[test]
fn test_multiple_of_power_of_5_32_case_4() {
    let value: u32 = 125; // 125 = 5^3
    let p: u32 = 3;
    assert_eq!(multiple_of_power_of_5_32(value, p), true);
}

#[test]
fn test_multiple_of_power_of_5_32_case_5() {
    let value: u32 = 0; // Edge case, should panic as per debug_assert
    let p: u32 = 0;
    let result = std::panic::catch_unwind(|| {
        multiple_of_power_of_5_32(value, p);
    });
    assert!(result.is_err());
}

