// Answer 0

#[test]
fn test_visit_post_start_line() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = ast::Assertion { span: Box::new(span), kind: ast::AssertionKind::StartLine };
    let mut translator = TranslatorI::new(&Translator { stack: RefCell::new(Vec::new()), flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }), utf8: true, line_terminator: b'\n' }, ".*");
    let ast = Ast::Assertion(Box::new(assertion));
    let _ = translator.visit_post(&ast);
}

#[test]
fn test_visit_post_end_line() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = ast::Assertion { span: Box::new(span), kind: ast::AssertionKind::EndLine };
    let mut translator = TranslatorI::new(&Translator { stack: RefCell::new(Vec::new()), flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }), utf8: true, line_terminator: b'\n' }, ".*");
    let ast = Ast::Assertion(Box::new(assertion));
    let _ = translator.visit_post(&ast);
}

#[test]
fn test_visit_post_word_boundary() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = ast::Assertion { span: Box::new(span), kind: ast::AssertionKind::WordBoundary };
    let mut translator = TranslatorI::new(&Translator { stack: RefCell::new(Vec::new()), flags: Cell::new(Flags { unicode: Some(true), ..Default::default() }), utf8: true, line_terminator: b'\n' }, ".*");
    let ast = Ast::Assertion(Box::new(assertion));
    let _ = translator.visit_post(&ast);
}

