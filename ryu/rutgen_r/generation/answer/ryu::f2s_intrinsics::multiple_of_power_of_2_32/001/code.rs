// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_case_1() {
    let value = 16; // 2^4
    let p = 4;
    assert!(multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_2() {
    let value = 32; // 2^5
    let p = 5;
    assert!(multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_3() {
    let value = 15; // Not a multiple of 2^4
    let p = 4;
    assert!(!multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_4() {
    let value = 0; // Edge case, should return true for all p
    let p = 4;
    assert!(multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_5() {
    let value = 8; // 2^3
    let p = 3;
    assert!(multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_6() {
    let value = 3; // Not a multiple of 2^2
    let p = 2;
    assert!(!multiple_of_power_of_2_32(value, p));
}

#[test]
fn test_multiple_of_power_of_2_32_case_7() {
    let value = 64; // 2^6
    let p = 6;
    assert!(multiple_of_power_of_2_32(value, p));
}

