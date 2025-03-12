// Answer 0

#[test]
fn test_hir_perl_byte_class_space_negated() {
    struct TestVisitor {
        flags: Flags,
        translator: Translator,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}
    }

    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), utf8: false, stack: RefCell::new(vec![]), line_terminator: b'\n' };

    let ast_class = ast::ClassPerl { 
        span: Span { start: 0, end: 1 }, 
        kind: ast::ClassPerlKind::Space, 
        negated: true 
    };

    let translator_i = TranslatorI::new(&translator, "pattern");

    let _ = translator_i.hir_perl_byte_class(&ast_class);
}

