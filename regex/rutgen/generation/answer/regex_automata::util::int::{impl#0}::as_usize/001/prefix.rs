// Answer 0

#[test]
fn test_as_usize_min() {
    let value: u8 = 0;
    value.as_usize();
}

#[test]
fn test_as_usize_mid() {
    let value: u8 = 128;
    value.as_usize();
}

#[test]
fn test_as_usize_max() {
    let value: u8 = 255;
    value.as_usize();
}

