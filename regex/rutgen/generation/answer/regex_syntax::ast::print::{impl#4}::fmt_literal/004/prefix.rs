// Answer 0

#[test]
fn test_fmt_literal_special_line_feed() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\u{0B}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_space() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_bell() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{07}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_form_feed() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\u{0C}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_tab() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_carriage_return() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_octal() {
    let mut buffer = String::new();
    let writer = Writer { wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Octal,
        c: '7',
    };
    writer.fmt_literal(&ast);
}

