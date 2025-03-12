// Answer 0

#[test]
fn test_visit_class_set_item_post_literal_ascii() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
            crlf: None,
        }),
        utf8: false,
        line_terminator: b'\n',
    };

    let mut visitor = TranslatorI::new(&translator, "test_pattern");

    let x = Literal {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: LiteralKind::Character,
        c: 'a',
    };

    let ast = ast::ClassSetItem::Literal(Box::new(x));

    visitor.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_b() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
            crlf: None,
        }),
        utf8: false,
        line_terminator: b'\n',
    };

    let mut visitor = TranslatorI::new(&translator, "test_pattern");

    let x = Literal {
        span: Span {
            start: Position(1),
            end: Position(2),
        },
        kind: LiteralKind::Character,
        c: 'b',
    };

    let ast = ast::ClassSetItem::Literal(Box::new(x));

    visitor.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_z() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
            crlf: None,
        }),
        utf8: false,
        line_terminator: b'\n',
    };

    let mut visitor = TranslatorI::new(&translator, "test_pattern");

    let x = Literal {
        span: Span {
            start: Position(2),
            end: Position(3),
        },
        kind: LiteralKind::Character,
        c: 'z',
    };

    let ast = ast::ClassSetItem::Literal(Box::new(x));

    visitor.visit_class_set_item_post(&ast).unwrap();
}

