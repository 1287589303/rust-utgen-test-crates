// Answer 0

#[test]
fn test_hir_capture_non_capturing() {
    struct MockVisitor {
        output: Result<Hir, Error>,
    }

    impl Visitor for MockVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            self.output
        }

        fn start(&mut self) {}
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(ast::NonCapturingGroup::default())),
        ast: Box::new(Ast::default()),
    };

    let expr = Hir::empty();

    let translator_i = TranslatorI::new(&translator, "a*b");

    let result = translator_i.hir_capture(&group, expr);
}

#[test]
fn test_hir_capture_non_capturing_with_non_empty_expr() {
    struct MockVisitor {
        output: Result<Hir, Error>,
    }

    impl Visitor for MockVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            self.output
        }

        fn start(&mut self) {}
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::NonCapturing(Box::new(ast::NonCapturingGroup::default())),
        ast: Box::new(Ast::default()),
    };

    let expr = Hir::literal(b"a");

    let translator_i = TranslatorI::new(&translator, "a*b");

    let result = translator_i.hir_capture(&group, expr);
}

