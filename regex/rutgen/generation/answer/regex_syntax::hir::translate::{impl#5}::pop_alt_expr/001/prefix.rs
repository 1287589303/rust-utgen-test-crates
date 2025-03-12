// Answer 0

#[test]
#[should_panic]
fn test_pop_alt_expr_empty_stack() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "");
    translator_i.pop_alt_expr();
}

#[test]
#[should_panic]
fn test_pop_alt_expr_alternation_frame() {
    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Alternation]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "");
    translator_i.pop_alt_expr();
}

