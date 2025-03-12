// Answer 0

#[test]
fn test_hir_dot_invalid_utf8() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "test");

    let _ = translator_i.hir_dot(span);
}

#[test]
fn test_hir_dot_invalid_utf8_with_different_lineterm() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\r',
    };
    let translator_i = TranslatorI::new(&translator, "test");

    let _ = translator_i.hir_dot(span);
}

