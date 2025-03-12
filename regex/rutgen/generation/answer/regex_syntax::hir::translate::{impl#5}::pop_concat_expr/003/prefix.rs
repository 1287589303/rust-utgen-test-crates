// Answer 0

#[test]
fn test_pop_concat_expr_with_alternation_frame() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "a|b");

    translator
        .stack
        .borrow_mut()
        .push(HirFrame::Alternation);
    translator
        .stack
        .borrow_mut()
        .push(HirFrame::Expr(Hir { kind: HirKind::Literal, props: Properties::default() }));

    let result = translator_i.pop_concat_expr();
}

#[test]
fn test_pop_concat_expr_with_another_alternation_frame() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "c|d");

    translator
        .stack
        .borrow_mut()
        .push(HirFrame::Alternation);
    translator
        .stack
        .borrow_mut()
        .push(HirFrame::Expr(Hir { kind: HirKind::Literal, props: Properties::default() }));

    let result = translator_i.pop_concat_expr();
}

