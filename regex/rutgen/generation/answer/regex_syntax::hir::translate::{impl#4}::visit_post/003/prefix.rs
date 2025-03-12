// Answer 0

#[test]
fn test_visit_post_concat_with_empty_expr() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: 0, end: 10 };
    let empty_ast = Ast::Concat(Box::new(vec![
        Ast::Literal(Box::new(Literal {
            span,
            kind: LiteralKind::Unicode('a'),
            c: 'a',
        })),
        Ast::Empty(Box::new(span.clone())),
        Ast::Empty(Box::new(span.clone())),
    ]));

    let mut visitor = TranslatorI::new(&translator, "test_pattern");
    visitor.visit_post(&empty_ast).unwrap();
}

#[test]
fn test_visit_post_concat_with_multiple_empty_expr() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: 0, end: 10 };
    let empty_ast = Ast::Concat(Box::new(vec![
        Ast::Empty(Box::new(span.clone())),
        Ast::Empty(Box::new(span.clone())),
        Ast::Literal(Box::new(Literal {
            span,
            kind: LiteralKind::Unicode('b'),
            c: 'b',
        })),
        Ast::Empty(Box::new(span.clone())),
    ]));

    let mut visitor = TranslatorI::new(&translator, "test_pattern");
    visitor.visit_post(&empty_ast).unwrap();
}

