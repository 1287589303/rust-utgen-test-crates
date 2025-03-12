// Answer 0

#[test]
fn test_unexpected_i8_negative() {
    let content = Content::I8(-128);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i8_zero() {
    let content = Content::I8(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_i8_positive() {
    let content = Content::I8(127);
    let _result = content.unexpected();
}

