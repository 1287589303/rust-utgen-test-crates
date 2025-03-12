// Answer 0

#[test]
fn test_visit_post_repetition_valid() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a*";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: Position::new(0), end: Position::new(3) };
    let repetition_op = ast::Repetition { op: ast::RepetitionKind::ZeroOrMore, span: span.clone(), greedy: true, ast: Box::new(ast::Literal(Box::new(ast::Literal { c: 'a', span }))) };
    let ast = Ast::Repetition(Box::new(repetition_op));
    
    visitor.push(HirFrame::Expr(Hir::literal(vec![b'a'])));
    visitor.push(HirFrame::Repetition);
    
    let _ = visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition_empty() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = ".*";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: Position::new(0), end: Position::new(3) };
    let repetition_op = ast::Repetition { op: ast::RepetitionKind::ZeroOrMore, span: span.clone(), greedy: true, ast: Box::new(ast::Literal(Box::new(ast::Literal { c: '.', span }))) };
    let ast = Ast::Repetition(Box::new(repetition_op));
    
    visitor.push(HirFrame::Expr(Hir::literal(vec![b'.'])));
    visitor.push(HirFrame::Repetition);
    
    let _ = visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition_greedy() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a+";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: Position::new(0), end: Position::new(3) };
    let repetition_op = ast::Repetition { op: ast::RepetitionKind::OneOrMore, span: span.clone(), greedy: true, ast: Box::new(ast::Literal(Box::new(ast::Literal { c: 'a', span }))) };
    let ast = Ast::Repetition(Box::new(repetition_op));
    
    visitor.push(HirFrame::Expr(Hir::literal(vec![b'a'])));
    visitor.push(HirFrame::Repetition);
    
    let _ = visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition_not_greedy() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a*?";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: Position::new(0), end: Position::new(4) };
    let repetition_op = ast::Repetition { op: ast::RepetitionKind::ZeroOrMore, span: span.clone(), greedy: false, ast: Box::new(ast::Literal(Box::new(ast::Literal { c: 'a', span }))) };
    let ast = Ast::Repetition(Box::new(repetition_op));
    
    visitor.push(HirFrame::Expr(Hir::literal(vec![b'a'])));
    visitor.push(HirFrame::Repetition);
    
    let _ = visitor.visit_post(&ast).unwrap();
}

