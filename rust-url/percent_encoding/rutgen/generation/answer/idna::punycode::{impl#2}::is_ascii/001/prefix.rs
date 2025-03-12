// Answer 0

#[test]
fn test_is_ascii_below_threshold() {
    let value: u8 = 0;
    value.is_ascii();
}

#[test]
fn test_is_ascii_at_threshold() {
    let value: u8 = 127;
    value.is_ascii();
}

#[test]
fn test_is_ascii_above_threshold() {
    let value: u8 = 128;
    value.is_ascii();
}

#[test]
fn test_is_ascii_upper_bound() {
    let value: u8 = 255;
    value.is_ascii();
}

