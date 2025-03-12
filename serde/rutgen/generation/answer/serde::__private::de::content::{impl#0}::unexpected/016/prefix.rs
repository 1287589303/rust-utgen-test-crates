// Answer 0

#[test]
fn test_unexpected_signed_i16_min() {
    let content = Content::I16(-32768);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_signed_i16_zero() {
    let content = Content::I16(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_signed_i16_max() {
    let content = Content::I16(32767);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_signed_i16_negative() {
    let content = Content::I16(-1);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_signed_i16_positive() {
    let content = Content::I16(1);
    let _result = content.unexpected();
}

