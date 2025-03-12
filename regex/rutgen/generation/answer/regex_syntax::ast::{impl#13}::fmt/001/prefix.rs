// Answer 0

#[test]
fn test_fmt_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(Box::new(span));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let flags = SetFlags { span: span.clone(), flags: Flags::new() };
    let ast = Ast::Flags(Box::new(flags));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Basic, c: 'a' };
    let ast = Ast::Literal(Box::new(literal));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(Box::new(span));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(Box::new(assertion));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_class_unicode() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let ast = Ast::ClassUnicode(Box::new(class_unicode));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_class_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_class_bracketed() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_bracketed = ClassBracketed { span: span.clone(), negated: true, kind: ClassSet::Normal };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Literal(Box::new(Literal { span, kind: LiteralKind::Basic, c: 'b' }))) };
    let ast = Ast::Repetition(Box::new(repetition));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let group = Group { span: span.clone(), kind: GroupKind::Capture, ast: Box::new(Ast::Literal(Box::new(Literal { span, kind: LiteralKind::Basic, c: 'c' }))) };
    let ast = Ast::Group(Box::new(group));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_alternation() {
    let span = Span { start: Position(0), end: Position(5) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Basic, c: 'd' }))] };
    let ast = Ast::Alternation(Box::new(alternation));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

#[test]
fn test_fmt_concat() {
    let span = Span { start: Position(0), end: Position(5) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Basic, c: 'e' }))] };
    let ast = Ast::Concat(Box::new(concat));
    let mut formatter = core::fmt::Formatter::new();
    let _ = fmt(&ast, &mut formatter);
}

