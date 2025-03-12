// Answer 0

#[test]
fn test_pop_concat_expr_with_expr_frame() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::empty()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_instance = TranslatorI::new(&translator, "test");
    translator_instance.push(HirFrame::Expr(Hir { kind: HirKind::Empty, props: Properties::default() }));
    translator_instance.push(HirFrame::Repetition);
    let result = translator_instance.pop_concat_expr();
}

#[test]
fn test_pop_concat_expr_with_literal_frame() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::empty()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_instance = TranslatorI::new(&translator, "test");
    translator_instance.push(HirFrame::Literal(vec![b'a', b'b']));
    translator_instance.push(HirFrame::Repetition);
    let result = translator_instance.pop_concat_expr();
}

#[test]
#[should_panic(expected = "expected expr or concat, got repetition")]
fn test_pop_concat_expr_with_repetition_frame() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::empty()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_instance = TranslatorI::new(&translator, "test");
    translator_instance.push(HirFrame::Repetition);
    let result = translator_instance.pop_concat_expr();
}

