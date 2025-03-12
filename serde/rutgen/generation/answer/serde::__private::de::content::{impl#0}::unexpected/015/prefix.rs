// Answer 0

#[test]
fn test_unexpected_with_i32_min() {
    let content = Content::I32(i32::MIN);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_with_i32_zero() {
    let content = Content::I32(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_with_i32_positive() {
    let content = Content::I32(12345);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_with_i32_max() {
    let content = Content::I32(i32::MAX);
    let _result = content.unexpected();
}

