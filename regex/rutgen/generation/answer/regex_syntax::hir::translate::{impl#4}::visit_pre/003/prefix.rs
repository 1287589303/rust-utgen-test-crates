// Answer 0

#[test]
fn test_visit_pre_alternation_empty() {
    struct TestVisitor {
        translator: Translator,
    }

    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.translator.stack.borrow_mut().pop().unwrap().into_hir())
        }
    }

    let empty_alternation = Ast::Alternation(Box::new(Alternation {
        span: Span::default(),
        asts: vec![],
    }));
    let mut visitor = TestVisitor {
        translator: Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags::default()),
            utf8: true,
            line_terminator: b'\n',
        },
    };

    visitor.visit_pre(&empty_alternation).unwrap();
}

