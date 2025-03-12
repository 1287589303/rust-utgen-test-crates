// Answer 0

#[test]
fn test_hir_capture_with_capture_index() {
    struct MockGroup {
        kind: ast::GroupKind,
    }

    let group = MockGroup {
        kind: ast::GroupKind::CaptureIndex(0),
    };

    let expr = Hir::empty();

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "test",
    };

    let _result = translator_i.hir_capture(&group, expr);
}

#[test]
fn test_hir_capture_with_capture_name() {
    struct MockGroup {
        kind: ast::GroupKind,
    }

    let group = MockGroup {
        kind: ast::GroupKind::CaptureName {
            name: CaptureName {
                span: Span::default(),
                name: "group_name".to_string(),
                index: 1,
            },
        },
    };

    let expr = Hir::empty();

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "test",
    };

    let _result = translator_i.hir_capture(&group, expr);
}

