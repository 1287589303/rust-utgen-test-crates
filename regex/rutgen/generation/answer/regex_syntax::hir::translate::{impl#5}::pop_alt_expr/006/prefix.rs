// Answer 0

#[test]
fn test_pop_alt_expr_with_repetition() {
    struct TestVisitor {
        output: Option<Hir>,
        err: Option<Error>,
    }

    impl Visitor for TestVisitor {
        type Output = Option<Hir>;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            if let Some(err) = self.err {
                Err(err)
            } else {
                Ok(self.output)
            }
        }

        fn start(&mut self) {}
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::empty()),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "a|b");

    translator_i.push(HirFrame::Repetition);

    let result = translator_i.pop_alt_expr();
}

