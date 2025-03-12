// Answer 0

#[test]
fn test_hex_literal_kind_x() {
    let kind = HexLiteralKind::X;
    let _result = kind.digits();
}

#[test]
fn test_hex_literal_kind_unicode_short() {
    let kind = HexLiteralKind::UnicodeShort;
    let _result = kind.digits();
}

#[test]
fn test_hex_literal_kind_unicode_long() {
    let kind = HexLiteralKind::UnicodeLong;
    let _result = kind.digits();
}

