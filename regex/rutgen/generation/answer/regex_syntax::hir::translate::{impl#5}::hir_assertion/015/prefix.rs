// Answer 0

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct TestVisitor {
        translator: Translator,
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };

    let visitor = TestVisitor { translator: translator.clone() };
    let _result = visitor.translator.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_word_boundary_start_unicode() {
    struct TestVisitor {
        translator: Translator,
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };

    let visitor = TestVisitor { translator: translator.clone() };
    let _result = visitor.translator.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_word_boundary_end_unicode() {
    struct TestVisitor {
        translator: Translator,
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };

    let visitor = TestVisitor { translator: translator.clone() };
    let _result = visitor.translator.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_word_boundary_negate_unicode() {
    struct TestVisitor {
        translator: Translator,
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let visitor = TestVisitor { translator: translator.clone() };
    let _result = visitor.translator.hir_assertion(&asst);
}

