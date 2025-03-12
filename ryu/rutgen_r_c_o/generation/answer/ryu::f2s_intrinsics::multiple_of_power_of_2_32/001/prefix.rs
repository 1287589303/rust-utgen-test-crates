// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_case1() {
    let value: u32 = 0b00000000_00000000_00000000_00000000;
    let p: u32 = 0;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case2() {
    let value: u32 = 0b11111111_11111111_11111111_11111111;
    let p: u32 = 31;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case3() {
    let value: u32 = 0b00000000_00000000_00000000_00000001;
    let p: u32 = 1;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case4() {
    let value: u32 = 0b00000000_00000000_00000000_00000010;
    let p: u32 = 1;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case5() {
    let value: u32 = 0b00000000_00000000_00000000_00000000;
    let p: u32 = 31;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case6() {
    let value: u32 = 0b00000000_00000000_00000000_00000000;
    let p: u32 = 15;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case7() {
    let value: u32 = 0b00000000_00000000_00000000_00001111;
    let p: u32 = 4;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case8() {
    let value: u32 = 0b00000000_00000000_00001111_00000000;
    let p: u32 = 8;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case9() {
    let value: u32 = 0b00000000_00000000_00001111_11111111;
    let p: u32 = 8;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case10() {
    let value: u32 = 0b11111111_11111111_11111111_11111111;
    let p: u32 = 0;
    multiple_of_power_of_2_32(value, p);
}

#[test]
fn test_multiple_of_power_of_2_32_case11() {
    let value: u32 = 0b10101010_10101010_10101010_10101010;
    let p: u32 = 3;
    multiple_of_power_of_2_32(value, p);
}

