// Answer 0

#[test]
fn test_low_u16_zero() {
    let value: u32 = 0;
    let result = value.low_u16();
}

#[test]
fn test_low_u16_max_u16() {
    let value: u32 = 65535;
    let result = value.low_u16();
}

#[test]
fn test_low_u16_above_max_u16() {
    let value: u32 = 65536;
    let result = value.low_u16();
}

#[test]
fn test_low_u16_max_u32() {
    let value: u32 = 4294967295;
    let result = value.low_u16();
}

