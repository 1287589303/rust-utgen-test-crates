// Answer 0

#[test]
fn test_is_not_empty_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let set_flags = SetFlags { span, flags: Flags::default() };
    let ast = Ast::flags(set_flags);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Character, c: 'a' };
    let ast = Ast::literal(literal);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::dot(span);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let ast = Ast::assertion(assertion);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_class_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter };
    let ast = Ast::class_unicode(class_unicode);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_class_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let ast = Ast::class_perl(class_perl);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_class_bracketed() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_bracketed = ClassBracketed { span, negated: false, kind: ClassSet::Normal };
    let ast = Ast::class_bracketed(class_bracketed);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition = Repetition { span, op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::empty(span)) };
    let ast = Ast::repetition(repetition);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let group = Group { span, kind: GroupKind::Regular, ast: Box::new(Ast::empty(span)) };
    let ast = Ast::group(group);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let alternation = Alternation { span, asts: vec![Ast::empty(span)] };
    let ast = Ast::alternation(alternation);
    let result = ast.is_empty();
}

#[test]
fn test_is_not_empty_concat() {
    let span = Span { start: Position(0), end: Position(1) };
    let concat = Concat { span, asts: vec![Ast::empty(span)] };
    let ast = Ast::concat(concat);
    let result = ast.is_empty();
}

