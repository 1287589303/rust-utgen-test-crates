// Answer 0

#[test]
fn test_hir_repetition_one_or_more() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { case_insensitive: None, multi_line: None, dot_matches_new_line: None, swap_greed: Some(false), unicode: None, crlf: None }),
        utf8: true,
        line_terminator: b'\n',
    };
    
    let pattern = "a+";
    let translator_instance = TranslatorI::new(&trans, pattern);

    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("a")),
    };

    let expr = Hir::literal(b"a");
    
    let result = translator_instance.hir_repetition(&rep, expr);
} 

#[test]
fn test_hir_repetition_one_or_more_non_greedy() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { case_insensitive: None, multi_line: None, dot_matches_new_line: None, swap_greed: Some(false), unicode: None, crlf: None }),
        utf8: true,
        line_terminator: b'\n',
    };

    let pattern = "a+";
    let translator_instance = TranslatorI::new(&trans, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        span: Span::default(),
        ast: Box::new(ast::Literal::new("a")),
    };

    let expr = Hir::literal(b"a");

    let result = translator_instance.hir_repetition(&rep, expr);
}

