// Answer 0

#[test]
fn test_multiple_of_power_of_2_case1() {
    let value: u64 = 8; // 0b1000
    let p: u32 = 3; // 2^3 = 8
    assert_eq!(multiple_of_power_of_2(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_case2() {
    let value: u64 = 20; // 0b10100
    let p: u32 = 2; // 2^2 = 4
    assert_eq!(multiple_of_power_of_2(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_case3() {
    let value: u64 = 16; // 0b10000
    let p: u32 = 4; // 2^4 = 16
    assert_eq!(multiple_of_power_of_2(value, p), true);
}

#[test]
fn test_multiple_of_power_of_2_case4() {
    let value: u64 = 0b11111111; // 255
    let p: u32 = 8; // 2^8 = 256
    assert_eq!(multiple_of_power_of_2(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_case5() {
    let value: u64 = 1; // 0b1
    let p: u32 = 1; // 2^1 = 2
    assert_eq!(multiple_of_power_of_2(value, p), false);
}

#[test]
fn test_multiple_of_power_of_2_case6() {
    let value: u64 = 32; // 0b100000
    let p: u32 = 5; // 2^5 = 32
    assert_eq!(multiple_of_power_of_2(value, p), true);
}

