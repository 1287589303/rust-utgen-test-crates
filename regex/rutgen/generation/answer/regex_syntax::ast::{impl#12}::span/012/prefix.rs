// Answer 0

#[test]
fn test_span_empty() {
    let span = Span {
        start: Position(0),
        end: Position(0),
    };
    let ast = Ast::empty(span);
    ast.span();
}

#[test]
fn test_span_flags() {
    let span = Span {
        start: Position(1),
        end: Position(2),
    };
    let flags = SetFlags {
        span,
        flags: Flags::default(),
    };
    let ast = Ast::flags(flags);
    ast.span();
}

#[test]
fn test_span_literal() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let literal = Literal {
        span,
        kind: LiteralKind::Character,
        c: 'a',
    };
    let ast = Ast::literal(literal);
    ast.span();
}

#[test]
fn test_span_dot() {
    let span = Span {
        start: Position(2),
        end: Position(3),
    };
    let ast = Ast::dot(span);
    ast.span();
}

#[test]
fn test_span_assertion() {
    let span = Span {
        start: Position(4),
        end: Position(5),
    };
    let assertion = Assertion {
        span,
        kind: AssertionKind::Start,
    };
    let ast = Ast::assertion(assertion);
    ast.span();
}

#[test]
fn test_span_class_unicode() {
    let span = Span {
        start: Position(6),
        end: Position(7),
    };
    let class_unicode = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::Letter,
    };
    let ast = Ast::class_unicode(class_unicode);
    ast.span();
}

#[test]
fn test_span_class_perl() {
    let span = Span {
        start: Position(8),
        end: Position(9),
    };
    let class_perl = ClassPerl {
        span,
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let ast = Ast::class_perl(class_perl);
    ast.span();
}

#[test]
fn test_span_class_bracketed() {
    let span = Span {
        start: Position(10),
        end: Position(11),
    };
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Normal,
    };
    let ast = Ast::class_bracketed(class_bracketed);
    ast.span();
}

#[test]
fn test_span_repetition() {
    let span = Span {
        start: Position(12),
        end: Position(13),
    };
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Ast::empty(span)),
    };
    let ast = Ast::repetition(repetition);
    ast.span();
}

#[test]
fn test_span_group() {
    let span = Span {
        start: Position(14),
        end: Position(15),
    };
    let group = Group {
        span,
        kind: GroupKind::Capture,
        ast: Box::new(Ast::empty(span)),
    };
    let ast = Ast::group(group);
    ast.span();
}

#[test]
fn test_span_alternation() {
    let span = Span {
        start: Position(16),
        end: Position(17),
    };
    let alternation = Alternation {
        span,
        asts: vec![Ast::empty(span), Ast::empty(span)],
    };
    let ast = Ast::alternation(alternation);
    ast.span();
}

#[test]
fn test_span_concat() {
    let span = Span {
        start: Position(18),
        end: Position(19),
    };
    let concat = Concat {
        span,
        asts: vec![Ast::empty(span), Ast::empty(span)],
    };
    let ast = Ast::concat(concat);
    ast.span();
}

