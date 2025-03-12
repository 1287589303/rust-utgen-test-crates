// Answer 0

#[test]
fn test_unexpected_u16_zero() {
    let content = Content::U16(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u16_mid_range() {
    let content = Content::U16(32768);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u16_max() {
    let content = Content::U16(65535);
    let _result = content.unexpected();
}

