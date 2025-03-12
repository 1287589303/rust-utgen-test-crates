// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a?";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("a".to_string().into_boxed_bytes());

    translator_instance.hir_repetition(&rep, expr);
}

#[test]
fn test_hir_repetition_zero_or_more() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a*";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("a".to_string().into_boxed_bytes());
    
    translator_instance.hir_repetition(&rep, expr);
}

#[test]
fn test_hir_repetition_one_or_more() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a+";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("a".to_string().into_boxed_bytes());
    
    translator_instance.hir_repetition(&rep, expr);
}

#[test]
fn test_hir_repetition_range_exactly() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a{2}";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(2)),
        },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("aa".to_string().into_boxed_bytes());
    
    translator_instance.hir_repetition(&rep, expr);
}

#[test]
fn test_hir_repetition_range_at_least() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a{2,}";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)),
        },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("aa".to_string().into_boxed_bytes());
    
    translator_instance.hir_repetition(&rep, expr);
}

#[test]
fn test_hir_repetition_range_bounded() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { swap_greed: Some(false), ..Default::default() }),
        utf8: true,
        line_terminator: b'\n',
    };
    let pattern = "a{2,4}";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 4)),
        },
        span: Span::default(),
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    let expr = Hir::literal("aa".to_string().into_boxed_bytes());
    
    translator_instance.hir_repetition(&rep, expr);
}

