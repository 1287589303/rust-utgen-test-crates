// Answer 0

#[test]
fn test_hir_ascii_unicode_class_alnum() {
    struct TestVisitor {
        flags: ast::Flags,
    }

    let ast = ast::ClassAscii {
        span: Span { start: 0, end: 255 },
        kind: ClassAsciiKind::Alnum,
        negated: false,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(ast.flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_instance = TranslatorI::new(&translator, "test");

    let _ = translator_instance.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_alpha() {
    struct TestVisitor {
        flags: ast::Flags,
    }

    let ast = ast::ClassAscii {
        span: Span { start: 0, end: 255 },
        kind: ClassAsciiKind::Alpha,
        negated: false,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(ast.flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_instance = TranslatorI::new(&translator, "test");

    let _ = translator_instance.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_digit() {
    struct TestVisitor {
        flags: ast::Flags,
    }

    let ast = ast::ClassAscii {
        span: Span { start: 0, end: 255 },
        kind: ClassAsciiKind::Digit,
        negated: false,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(ast.flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_instance = TranslatorI::new(&translator, "test");

    let _ = translator_instance.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_space() {
    struct TestVisitor {
        flags: ast::Flags,
    }

    let ast = ast::ClassAscii {
        span: Span { start: 0, end: 255 },
        kind: ClassAsciiKind::Space,
        negated: true,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(ast.flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_instance = TranslatorI::new(&translator, "test");

    let _ = translator_instance.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_punct() {
    struct TestVisitor {
        flags: ast::Flags,
    }

    let ast = ast::ClassAscii {
        span: Span { start: 0, end: 255 },
        kind: ClassAsciiKind::Punct,
        negated: false,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(ast.flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_instance = TranslatorI::new(&translator, "test");

    let _ = translator_instance.hir_ascii_unicode_class(&ast);
}

