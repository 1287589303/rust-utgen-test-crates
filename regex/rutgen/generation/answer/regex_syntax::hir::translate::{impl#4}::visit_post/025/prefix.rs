// Answer 0

#[test]
fn test_visit_post_literal_with_case_fold_none() {
    let literal = ast::Literal { span: Span { start: Position(0), end: Position(1) }, c: 'a' };
    let ast = Ast::Literal(Box::new(literal));
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut translator_i = TranslatorI::new(&translator, "a");
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_literal_with_case_fold_some() {
    let literal = ast::Literal { span: Span { start: Position(0), end: Position(1) }, c: 'A' };
    let ast = Ast::Literal(Box::new(literal));
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            multi_line: Some(false),
            dot_matches_new_line: Some(false),
            swap_greed: Some(false),
            unicode: Some(true),
            crlf: Some(false),
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut translator_i = TranslatorI::new(&translator, "A");
    translator_i.visit_post(&ast).unwrap();
}

