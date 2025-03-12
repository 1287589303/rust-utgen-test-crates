// Answer 0

#[test]
fn test_visit_post_class_bracketed() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let class_set_item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span: Span::new(0, 5),
        negated: false,
        kind: ast::ClassSet::Union(vec![ast::ClassSetItem::Literal(Box::new(ast::Literal {
            span: Span::new(1, 2),
            kind: ast::LiteralKind::Verbatim,
            c: 'a',
        }))]),
    }));

    let class_bracketed = ast::ClassBracketed {
        span: Span::new(0, 6),
        negated: false,
        kind: ast::ClassSet::Union(vec![class_set_item]),
    };

    let ast = ast::Ast::ClassBracketed(Box::new(class_bracketed));

    writer.visit_post(&ast).unwrap();
}

