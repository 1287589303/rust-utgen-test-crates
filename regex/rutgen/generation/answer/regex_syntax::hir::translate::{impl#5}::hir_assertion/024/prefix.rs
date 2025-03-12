// Answer 0

#[test]
fn test_hir_assertion_start_line_crlf_true() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
            crlf: Some(true),
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };

    let translator_i = TranslatorI::new(&trans, "");

    let _ = translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_start_line_crlf_false() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
            crlf: Some(false),
        }),
        utf8: true,
        line_terminator: b'\n',
    };

    let asst = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };

    let translator_i = TranslatorI::new(&trans, "");

    let _ = translator_i.hir_assertion(&asst);
}

