// Answer 0

#[test]
fn test_visit_post_concat_with_non_empty_expressions() {
    let mut translator = Translator::default();
    let ast = Ast::Concat(Box::new(Concat {
        expressions: vec![
            Ast::Literal(Box::new(Literal { span: Span::new(0, 1), c: 'a' })),
            Ast::ClassUnicode(Box::new(ClassUnicode { span: Span::new(1, 2), negated: false, kind: ClassUnicodeKind::OneLetter('L') })),
        ],
    }));

    let mut visitor = TranslatorI::new(&translator, "a[L]");
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat_with_mixed_expressions() {
    let mut translator = Translator::default();
    let ast = Ast::Concat(Box::new(Concat {
        expressions: vec![
            Ast::Repetition(Box::new(Repetition { span: Span::new(0, 1), op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Literal(Box::new(Literal { span: Span::new(1, 2), c: 'b' }))) })),
            Ast::Alternation(Box::new(Alternation { expressions: vec![Ast::Literal(Box::new(Literal { span: Span::new(2, 3), c: 'c' }))] })),
        ],
    }));

    let mut visitor = TranslatorI::new(&translator, "b|c");
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat_with_multiple_non_empty_hir() {
    let mut translator = Translator::default();
    let ast = Ast::Concat(Box::new(Concat {
        expressions: vec![
            Ast::ClassBracketed(Box::new(ClassBracketed { span: Span::new(0, 3), negated: false, kind: ClassSet::Normal })),
            Ast::Repetition(Box::new(Repetition { span: Span::new(3, 5), op: RepetitionOp::OneOrMore, greedy: true, ast: Box::new(Ast::Literal(Box::new(Literal { span: Span::new(5, 6), c: 'd' }))) })),
        ],
    }));

    let mut visitor = TranslatorI::new(&translator, "[abc]d+");
    visitor.visit_post(&ast).unwrap();
}

