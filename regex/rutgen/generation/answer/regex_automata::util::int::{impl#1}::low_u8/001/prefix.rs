// Answer 0

#[test]
fn test_low_u8_zero() {
    let value: u16 = 0;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_mid() {
    let value: u16 = 255;
    let result = value.low_u8();
}

#[test]
fn test_low_u8_max() {
    let value: u16 = 65535;
    let result = value.low_u8();
}

