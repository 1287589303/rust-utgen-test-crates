// Answer 0

#[test]
fn test_u8_zero() {
    let byte = 0u8;
    let unit = Unit::u8(byte);
}

#[test]
fn test_u8_mid_range() {
    let byte = 128u8;
    let unit = Unit::u8(byte);
}

#[test]
fn test_u8_max() {
    let byte = 255u8;
    let unit = Unit::u8(byte);
}

#[test]
fn test_u8_min() {
    let byte = 1u8;
    let unit = Unit::u8(byte);
}

#[test]
fn test_u8_second_last() {
    let byte = 254u8;
    let unit = Unit::u8(byte);
}

