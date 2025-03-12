// Answer 0

#[test]
fn test_hir_perl_unicode_class_digit() {
    struct TestVisitor {
        output: (),
        err: (),
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: 0, end: 5 };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let class_unicode = translator.hir_perl_unicode_class(&ast_class).unwrap();
}

#[test]
fn test_hir_perl_unicode_class_space() {
    struct TestVisitor {
        output: (),
        err: (),
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: 0, end: 5 };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let class_unicode = translator.hir_perl_unicode_class(&ast_class).unwrap();
}

#[test]
fn test_hir_perl_unicode_class_word() {
    struct TestVisitor {
        output: (),
        err: (),
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: 0, end: 5 };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let class_unicode = translator.hir_perl_unicode_class(&ast_class).unwrap();
}

