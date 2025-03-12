// Answer 0

#[test]
fn test_literal_byte_verbatim() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::Verbatim,
        c: 'a',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_meta() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::Meta,
        c: '*',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_superfluous() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::Superfluous,
        c: '%',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_octal() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::Octal,
        c: '7',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_hex_fixed_y() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::HexFixed(HexLiteralKind::Y),
        c: 'y',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_hex_brace_x() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::HexBrace(HexLiteralKind::X),
        c: 'a',
    };
    literal.byte();
}

#[test]
fn test_literal_byte_special() {
    let literal = Literal {
        span: Span { start: Position::default(), end: Position::default() },
        kind: LiteralKind::Special(SpecialLiteralKind::Newline),
        c: '\n',
    };
    literal.byte();
}

