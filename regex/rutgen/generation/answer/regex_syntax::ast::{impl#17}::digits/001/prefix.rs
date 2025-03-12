// Answer 0

#[test]
fn test_digits_unicode_long() {
    let kind = HexLiteralKind::UnicodeLong;
    let result = kind.digits();
}

#[test]
fn test_digits_unicode_short() {
    let kind = HexLiteralKind::UnicodeShort;
    let result = kind.digits();
}

#[test]
fn test_digits_x() {
    let kind = HexLiteralKind::X;
    let result = kind.digits();
}

