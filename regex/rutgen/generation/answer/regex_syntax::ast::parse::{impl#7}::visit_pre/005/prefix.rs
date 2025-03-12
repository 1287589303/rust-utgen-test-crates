// Answer 0

#[test]
fn test_visit_pre_class_bracketed_valid() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_bracketed = ClassBracketed { span, negated: false, kind: ClassSet::Normal };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));

    let parser = ParserI { parser: Parser { nest_limit: 5, /* other fields */ }, pattern: "[a-z]" };
    let mut nest_limiter = NestLimiter::new(&parser);

    let _ = nest_limiter.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_depth_limit() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_bracketed = ClassBracketed { span, negated: false, kind: ClassSet::Normal };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));

    let parser = ParserI { parser: Parser { nest_limit: 1, /* other fields */ }, pattern: "[a-z]" };
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.depth = 1; // Manually set depth to match limit

    let _ = nest_limiter.visit_pre(&ast);
}

#[test]
#[should_panic]
fn test_visit_pre_class_bracketed_exceed_depth_limit() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_bracketed = ClassBracketed { span, negated: false, kind: ClassSet::Normal };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));

    let parser = ParserI { parser: Parser { nest_limit: 1, /* other fields */ }, pattern: "[a-z]" };
    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.depth = 2; // Manually set depth to exceed limit

    let _ = nest_limiter.visit_pre(&ast);
}

