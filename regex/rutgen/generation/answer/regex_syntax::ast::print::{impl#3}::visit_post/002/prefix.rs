// Answer 0

#[test]
fn test_visit_post_with_alternation() {
    let alternation_ast = ast::Alternation(Box::new(ast::Concat(Box::new(ast::Empty(Box::new(Span::default()))))));
    let mut writer = Writer { wtr: String::new() };
    writer.visit_post(&Ast::Alternation(Box::new(alternation_ast))).unwrap();
}

#[test]
fn test_visit_post_with_alternation_empty() {
    let alternation_ast = ast::Alternation(Box::new(ast::Empty(Box::new(Span::default()))));
    let mut writer = Writer { wtr: String::new() };
    writer.visit_post(&Ast::Alternation(Box::new(alternation_ast))).unwrap();
}

#[test]
fn test_visit_post_with_alternation_non_empty() {
    let concat_ast = ast::Concat(Box::new(ast::Literal(Box::new(ast::Literal { 
        span: Span::default(), 
        kind: ast::LiteralKind::Verbatim, 
        c: 'a' 
    }))));
    let alternation_ast = ast::Alternation(Box::new(concat_ast));
    let mut writer = Writer { wtr: String::new() };
    writer.visit_post(&Ast::Alternation(Box::new(alternation_ast))).unwrap();
}

