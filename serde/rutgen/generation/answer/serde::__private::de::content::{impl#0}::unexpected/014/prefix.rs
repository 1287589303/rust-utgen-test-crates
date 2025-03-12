// Answer 0

#[test]
fn test_unexpected_i64_min() {
    let content = Content::I64(i64::MIN);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i64_zero() {
    let content = Content::I64(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i64_max() {
    let content = Content::I64(i64::MAX);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i64_negative() {
    let content = Content::I64(-12345);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i64_positive() {
    let content = Content::I64(12345);
    let _result = content.unexpected();
}

