// Answer 0

#[test]
fn test_high_u8_zero() {
    let value: u16 = 0;
    value.high_u8();
}

#[test]
fn test_high_u8_255() {
    let value: u16 = 255;
    value.high_u8();
}

#[test]
fn test_high_u8_256() {
    let value: u16 = 256;
    value.high_u8();
}

#[test]
fn test_high_u8_511() {
    let value: u16 = 511;
    value.high_u8();
}

#[test]
fn test_high_u8_65535() {
    let value: u16 = 65535;
    value.high_u8();
}

#[test]
fn test_high_u8_mid_value() {
    let value: u16 = 32768;
    value.high_u8();
}

