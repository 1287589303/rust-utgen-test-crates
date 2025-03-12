// Answer 0

#[test]
fn test_visit_class_set_item_post_literal_unicode() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let class_set_item = ast::ClassSetItem::Literal(Literal {
        span: Span {
            start: Position::default(),
            end: Position::default(),
        },
        kind: LiteralKind::Character,
        c: 'a',
    });

    let mut visitor = TranslatorI::new(&translator, "");

    visitor.push(HirFrame::ClassUnicode(ClassUnicode::empty()));

    let result = visitor.visit_class_set_item_post(&class_set_item);

    assert!(result.is_ok());
}

#[test]
fn test_visit_class_set_item_post_literal_unicode_multiple() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let class_set_item = ast::ClassSetItem::Literal(Literal {
        span: Span {
            start: Position::default(),
            end: Position::default(),
        },
        kind: LiteralKind::Character,
        c: 'b',
    });

    let mut visitor = TranslatorI::new(&translator, "");

    visitor.push(HirFrame::ClassUnicode(ClassUnicode::empty()));

    let result = visitor.visit_class_set_item_post(&class_set_item);

    assert!(result.is_ok());
}

#[test]
fn test_visit_class_set_item_post_literal_unicode_edge_case() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let class_set_item = ast::ClassSetItem::Literal(Literal {
        span: Span {
            start: Position::default(),
            end: Position::default(),
        },
        kind: LiteralKind::Character,
        c: 'z',
    });

    let mut visitor = TranslatorI::new(&translator, "");

    visitor.push(HirFrame::ClassUnicode(ClassUnicode::empty()));

    let result = visitor.visit_class_set_item_post(&class_set_item);

    assert!(result.is_ok());
}

