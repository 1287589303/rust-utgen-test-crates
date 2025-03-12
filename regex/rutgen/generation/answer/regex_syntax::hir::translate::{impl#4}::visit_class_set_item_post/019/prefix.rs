// Answer 0

#[test]
fn test_visit_class_set_item_post_range_non_unicode() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let start_literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::Char,
        c: 'a',
    };

    let end_literal = Literal {
        span: Span { start: Position(2), end: Position(3) },
        kind: LiteralKind::Char,
        c: 'z',
    };

    let class_set_item = ast::ClassSetItem::Range(ClassSetRange {
        span: Span { start: Position(0), end: Position(4) },
        start: start_literal,
        end: end_literal,
    });

    let translator_i = TranslatorI::new(&translator, "pattern");
    translator_i.visit_class_set_item_post(&class_set_item).unwrap();
}

