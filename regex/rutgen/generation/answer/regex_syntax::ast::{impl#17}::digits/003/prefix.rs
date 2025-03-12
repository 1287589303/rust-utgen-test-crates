// Answer 0

#[test]
fn test_hex_literal_kind_x_digits() {
    let hex_literal = HexLiteralKind::X;
    let result = hex_literal.digits();
}

#[test]
fn test_hex_literal_kind_unicode_short_digits() {
    let hex_literal = HexLiteralKind::UnicodeShort;
    let result = hex_literal.digits();
}

#[test]
fn test_hex_literal_kind_unicode_long_digits() {
    let hex_literal = HexLiteralKind::UnicodeLong;
    let result = hex_literal.digits();
}

