// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_false() {
    struct TestVisitor {
        trans: Translator,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator_i = TranslatorI::new(&trans, "");

    let result = translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_ends() {
    struct TestVisitor {
        trans: Translator,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };

    let translator_i = TranslatorI::new(&trans, "");

    let result = translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_starts() {
    struct TestVisitor {
        trans: Translator,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let mut trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };

    let translator_i = TranslatorI::new(&trans, "");

    let result = translator_i.hir_assertion(&asst);
}

