// Answer 0

#[test]
fn test_check_empty_ast() {
    let span = Span::default();
    let ast = Ast::Empty(Box::new(span));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_flags_ast() {
    let span = Span::default();
    let set_flags = SetFlags::default();
    let ast = Ast::Flags(Box::new(set_flags));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_literal_ast() {
    let span = Span::default();
    let literal = Literal::default();
    let ast = Ast::Literal(Box::new(literal));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_dot_ast() {
    let span = Span::default();
    let ast = Ast::Dot(Box::new(span));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_assertion_ast() {
    let span = Span::default();
    let assertion = Assertion::default();
    let ast = Ast::Assertion(Box::new(assertion));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_class_unicode_ast() {
    let span = Span::default();
    let class_unicode = ClassUnicode::default();
    let ast = Ast::ClassUnicode(Box::new(class_unicode));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_class_perl_ast() {
    let span = Span::default();
    let class_perl = ClassPerl::default();
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_class_bracketed_ast() {
    let span = Span::default();
    let class_bracketed = ClassBracketed::default();
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_repetition_ast() {
    let span = Span::default();
    let repetition = Repetition::default();
    let ast = Ast::Repetition(Box::new(repetition));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_group_ast() {
    let span = Span::default();
    let group = Group::default();
    let ast = Ast::Group(Box::new(group));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_alternation_ast() {
    let span = Span::default();
    let alternation = Alternation::default();
    let ast = Ast::Alternation(Box::new(alternation));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_concat_ast() {
    let span = Span::default();
    let concat = Concat::default();
    let ast = Ast::Concat(Box::new(concat));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.check(&ast);
}

#[test]
fn test_check_depth_within_nest_limit() {
    let span = Span::default();
    let ast = Ast::Group(Box::new(Group::default()));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.increment_depth(&span);
    let _ = nest_limiter.check(&ast);
}

#[test]
#[should_panic]
fn test_check_exceed_nest_limit() {
    let span = Span::default();
    let ast = Ast::Group(Box::new(Group::default()));
    let parser = ParserI { parser: &Parser::default(), pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser);
    for _ in 0..parser.nest_limit + 1 {
        let _ = nest_limiter.increment_depth(&span);
    }
    let _ = nest_limiter.check(&ast);
}

