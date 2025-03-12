// Answer 0

#[test]
fn test_hir_assertion_word_boundary_start() {
    struct TestVisitor {
        flags: Flags,
    }

    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(Hir::empty())
        }

        fn start(&mut self) {}
    }

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let ast_assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };

    let translator_i = TranslatorI::new(&trans, "test_pattern");
    let _ = translator_i.hir_assertion(&ast_assertion);
}

#[test]
fn test_hir_assertion_word_boundary_start_angle() {
    struct TestVisitor {
        flags: Flags,
    }

    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(Hir::empty())
        }

        fn start(&mut self) {}
    }

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let ast_assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };

    let translator_i = TranslatorI::new(&trans, "test_pattern");
    let _ = translator_i.hir_assertion(&ast_assertion);
}

