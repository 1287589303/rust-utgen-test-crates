// Answer 0

#[test]
fn test_multiple_of_power_of_2_with_value_1_and_p_0() {
    let value: u64 = 1;
    let p: u32 = 0;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_2_and_p_1() {
    let value: u64 = 2;
    let p: u32 = 1;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_4_and_p_2() {
    let value: u64 = 4;
    let p: u32 = 2;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_8_and_p_3() {
    let value: u64 = 8;
    let p: u32 = 3;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_16_and_p_4() {
    let value: u64 = 16;
    let p: u32 = 4;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_32_and_p_5() {
    let value: u64 = 32;
    let p: u32 = 5;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_64_and_p_6() {
    let value: u64 = 64;
    let p: u32 = 6;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_128_and_p_7() {
    let value: u64 = 128;
    let p: u32 = 7;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_value_3_and_p_2() {
    let value: u64 = 3;
    let p: u32 = 2;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_with_large_value_and_p_63() {
    let value: u64 = 0xFFFFFFFFFFFFFFFF;
    let p: u32 = 63;
    multiple_of_power_of_2(value, p);
}

