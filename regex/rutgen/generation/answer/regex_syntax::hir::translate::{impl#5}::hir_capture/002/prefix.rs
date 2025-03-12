// Answer 0

#[test]
fn test_hir_capture_with_capture_name() {
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName {
            name: CaptureName {
                span: Span::default(),
                name: "valid_name".to_string(),
                index: 1,
            },
            ..Default::default()
        },
        ast: Box::new(Ast::default()),
    };
    
    let expr = Hir::empty(); // valid Hir object

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "some_pattern");

    translator_i.hir_capture(&group, expr);
}

#[test]
fn test_hir_capture_with_capture_name_index_zero() {
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName {
            name: CaptureName {
                span: Span::default(),
                name: "name_zero".to_string(),
                index: 0,
            },
            ..Default::default()
        },
        ast: Box::new(Ast::default()),
    };
    
    let expr = Hir::empty(); // valid Hir object

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "another_pattern");

    translator_i.hir_capture(&group, expr);
}

#[test]
fn test_hir_capture_with_capture_name_large_index() {
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName {
            name: CaptureName {
                span: Span::default(),
                name: "large_index_name".to_string(),
                index: 10000,
            },
            ..Default::default()
        },
        ast: Box::new(Ast::default()),
    };
    
    let expr = Hir::empty(); // valid Hir object

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "pattern_with_large_index");

    translator_i.hir_capture(&group, expr);
}

