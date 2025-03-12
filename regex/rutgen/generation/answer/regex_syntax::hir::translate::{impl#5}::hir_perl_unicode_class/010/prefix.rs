// Answer 0

#[test]
fn test_hir_perl_unicode_class_digit_negated() {
    struct TestVisitor {
        flags: Cell<Flags>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let flags = Flags { unicode: Some(false), ..Default::default() };
    let visitor = TestVisitor { flags: Cell::new(flags) };
    let ast_class = ast::ClassPerl {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let result = visitor.hir_perl_unicode_class(&ast_class);
}

#[test]
fn test_hir_perl_unicode_class_space_negated() {
    struct TestVisitor {
        flags: Cell<Flags>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let flags = Flags { unicode: Some(false), ..Default::default() };
    let visitor = TestVisitor { flags: Cell::new(flags) };
    let ast_class = ast::ClassPerl {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let result = visitor.hir_perl_unicode_class(&ast_class);
}

#[test]
fn test_hir_perl_unicode_class_word_not_negated() {
    struct TestVisitor {
        flags: Cell<Flags>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let flags = Flags { unicode: Some(false), ..Default::default() };
    let visitor = TestVisitor { flags: Cell::new(flags) };
    let ast_class = ast::ClassPerl {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = visitor.hir_perl_unicode_class(&ast_class);
}

