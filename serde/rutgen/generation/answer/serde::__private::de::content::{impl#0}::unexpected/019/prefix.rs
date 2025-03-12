// Answer 0

#[test]
fn test_unexpected_u32_min() {
    let content = Content::U32(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u32_mid() {
    let content = Content::U32(2147483648);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u32_max() {
    let content = Content::U32(4294967295);
    let _result = content.unexpected();
}

