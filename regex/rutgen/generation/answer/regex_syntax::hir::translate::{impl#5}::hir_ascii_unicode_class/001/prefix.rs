// Answer 0

#[test]
fn test_hir_ascii_unicode_class_with_alnum_negated() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Alnum,
        negated: true,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "a1");
    let _ = translator_i.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_with_alpha_negated() {
    let span = Span { start: Position(1), end: Position(2) };
    let ast = ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Alpha,
        negated: true,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "a1");
    let _ = translator_i.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_with_digit_negated() {
    let span = Span { start: Position(2), end: Position(3) };
    let ast = ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Digit,
        negated: true,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "1a");
    let _ = translator_i.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_with_asci_negated() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Ascii,
        negated: true,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "byte");
    let _ = translator_i.hir_ascii_unicode_class(&ast);
}

#[test]
fn test_hir_ascii_unicode_class_with_space_not_negated() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = ast::ClassAscii {
        span,
        kind: ClassAsciiKind::Space,
        negated: false,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "   ");
    let _ = translator_i.hir_ascii_unicode_class(&ast);
}

