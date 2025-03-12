// Answer 0

#[test]
fn test_hir_unicode_class_one_letter_unicode_not_allowed() {
    struct TestVisitor {
        flags: Flags,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
        
        fn flags(&self) -> Flags {
            self.flags
        }
    }

    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let visitor = TestVisitor { flags };

    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };

    let result = visitor.hir_unicode_class(&ast_class);
}

#[test]
fn test_hir_unicode_class_named_unicode_not_allowed() {
    struct TestVisitor {
        flags: Flags,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
        
        fn flags(&self) -> Flags {
            self.flags
        }
    }

    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let visitor = TestVisitor { flags };

    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::Named("L"),
    };

    let result = visitor.hir_unicode_class(&ast_class);
}

#[test]
fn test_hir_unicode_class_named_value_unicode_not_allowed() {
    struct TestVisitor {
        flags: Flags,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
        
        fn flags(&self) -> Flags {
            self.flags
        }
    }

    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let visitor = TestVisitor { flags };

    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue { name: "Script".to_string(), value: "Latin".to_string() },
    };

    let result = visitor.hir_unicode_class(&ast_class);
}

