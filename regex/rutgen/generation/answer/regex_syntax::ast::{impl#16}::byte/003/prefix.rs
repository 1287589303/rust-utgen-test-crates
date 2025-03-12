// Answer 0

#[test]
fn test_byte_with_valid_hex_fixed() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'a', // ASCII character within u8 range
    };
    literal.byte(); // Expected to return Some(u8)
}

#[test]
fn test_byte_with_valid_hex_fixed_boundary_min() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '\0', // Minimum valid Unicode scalar value
    };
    literal.byte(); // Expected to return Some(0)
}

#[test]
fn test_byte_with_valid_hex_fixed_boundary_max() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'ÿ', // Maximum valid Unicode scalar value that fits in u8
    };
    literal.byte(); // Expected to return Some(255)
}

#[test]
fn test_byte_with_valid_hex_fixed_high_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '0', // Valid hexadecimal digit within u8 range
    };
    literal.byte(); // Expected to return Some(48)
}

