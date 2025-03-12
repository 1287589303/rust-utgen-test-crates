// Answer 0

#[test]
fn test_is_ascii_below_threshold() {
    let value: u8 = 0x7F; // 127, which is below 128
    assert!(value.is_ascii());
}

#[test]
fn test_is_ascii_at_threshold() {
    let value: u8 = 0x80; // 128, which is not below 128
    assert!(!value.is_ascii());
}

#[test]
fn test_is_ascii_above_threshold() {
    let value: u8 = 0xFF; // 255, which is above 128
    assert!(!value.is_ascii());
}

#[test]
fn test_is_ascii_zero() {
    let value: u8 = 0x00; // 0, which is below 128
    assert!(value.is_ascii());
}

#[test]
fn test_is_ascii_mid_range() {
    let value: u8 = 0x4A; // 74, which is below 128
    assert!(value.is_ascii());
}

