// Answer 0

#[test]
#[should_panic]
fn test_visit_post_literal_to_scalar_err_unicode() {
    struct TestVisitor<'t, 'p> {
        trans: Translator,
        pattern: &'p str,
    }

    let pattern = ".*";
    let flags = Flags { unicode: Some(false), case_insensitive: None, ..Flags::default() };
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut visitor = TestVisitor { trans, pattern };

    let span = Span { start: Position(0), end: Position(1) };
    let literal = ast::Literal { span: Box::new(span), c: '€' }; // Invalid byte representation
    let ast = Ast::Literal(Box::new(literal));

    visitor.visit_post(&ast).unwrap();
}

#[test]
#[should_panic]
fn test_visit_post_literal_to_scalar_err_ascii() {
    struct TestVisitor<'t, 'p> {
        trans: Translator,
        pattern: &'p str,
    }

    let pattern = ".*";
    let flags = Flags { unicode: Some(false), case_insensitive: None, ..Flags::default() };
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let mut visitor = TestVisitor { trans, pattern };

    let span = Span { start: Position(0), end: Position(1) };
    let literal = ast::Literal { span: Box::new(span), c: '\u{80}' }; // Invalid ASCII byte representation
    let ast = Ast::Literal(Box::new(literal));

    visitor.visit_post(&ast).unwrap();
}

