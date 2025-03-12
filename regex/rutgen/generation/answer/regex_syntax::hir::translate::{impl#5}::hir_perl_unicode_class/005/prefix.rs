// Answer 0

#[test]
fn test_hir_perl_unicode_class_space_negated() {
    struct TestVisitor {
        translator: Translator,
        ast_class: ast::ClassPerl,
    }
    
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    
    let translator = Translator {
        flags: Cell::new(flags),
        stack: RefCell::new(vec![]),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: Position(0), end: Position(10) };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let visitor = TestVisitor {
        translator,
        ast_class,
    };

    let _result = visitor.translator.hir_perl_unicode_class(&visitor.ast_class);
}

#[test]
fn test_hir_perl_unicode_class_digit_negated() {
    struct TestVisitor {
        translator: Translator,
        ast_class: ast::ClassPerl,
    }
    
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    
    let translator = Translator {
        flags: Cell::new(flags),
        stack: RefCell::new(vec![]),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: Position(0), end: Position(10) };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let visitor = TestVisitor {
        translator,
        ast_class,
    };

    let _result = visitor.translator.hir_perl_unicode_class(&visitor.ast_class);
}

#[test]
fn test_hir_perl_unicode_class_word_negated() {
    struct TestVisitor {
        translator: Translator,
        ast_class: ast::ClassPerl,
    }
    
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    
    let translator = Translator {
        flags: Cell::new(flags),
        stack: RefCell::new(vec![]),
        utf8: true,
        line_terminator: b'\n',
    };

    let span = Span { start: Position(0), end: Position(10) };
    let ast_class = ast::ClassPerl {
        span,
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };

    let visitor = TestVisitor {
        translator,
        ast_class,
    };

    let _result = visitor.translator.hir_perl_unicode_class(&visitor.ast_class);
}

