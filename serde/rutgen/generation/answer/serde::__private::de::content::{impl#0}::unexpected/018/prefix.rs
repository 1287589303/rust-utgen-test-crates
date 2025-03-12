// Answer 0

#[test]
fn test_unexpected_u64_zero() {
    let content = Content::U64(0);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u64_min() {
    let content = Content::U64(1);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_u64_max() {
    let content = Content::U64(18446744073709551615);
    let _result = content.unexpected();
}

