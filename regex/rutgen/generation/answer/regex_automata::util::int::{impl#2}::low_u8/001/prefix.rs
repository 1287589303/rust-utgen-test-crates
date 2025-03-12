// Answer 0

#[test]
fn test_low_u8_zero() {
    let value: u32 = 0;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_one() {
    let value: u32 = 1;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_max_u8() {
    let value: u32 = 255;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_over_max_u8() {
    let value: u32 = 256;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_half_of_max_u32() {
    let value: u32 = 2147483647;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_max_u32() {
    let value: u32 = 4294967295;
    let result = value.low_u8();
}

