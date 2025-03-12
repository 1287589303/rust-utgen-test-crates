// Answer 0

#[test]
fn test_multiple_of_power_of_5_case1() {
    let value: u64 = 25; // 5^2
    let p: u32 = 2;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case2() {
    let value: u64 = 30; // 5^1 factor
    let p: u32 = 1;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case3() {
    let value: u64 = 5; // 5^1
    let p: u32 = 1;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case4() {
    let value: u64 = 1; // no factors of 5
    let p: u32 = 1;
    assert!(!multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case5() {
    let value: u64 = 125; // 5^3
    let p: u32 = 3;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case6() {
    let value: u64 = 1000; // 5^3 factor (1000 = 5^3 * 2^3)
    let p: u32 = 3;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_case7() {
    let value: u64 = 80; // 5^0 factor (80 = 5^0 * 2^4)
    let p: u32 = 1;
    assert!(!multiple_of_power_of_5(value, p));
}

