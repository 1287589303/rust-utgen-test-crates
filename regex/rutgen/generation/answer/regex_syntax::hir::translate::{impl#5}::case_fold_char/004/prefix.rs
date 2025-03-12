// Answer 0

#[test]
fn test_case_fold_char_unicode_no_overlap() {
    let span = Span { start: Position(0), end: Position(1) };
    
    let flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: Some(true),
        crlf: None,
    };
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "test");

    let result = translator_i.case_fold_char(span, 'ß');
}

#[test]
fn test_case_fold_char_unicode_no_overlap_numeric() {
    let span = Span { start: Position(0), end: Position(1) };
    
    let flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: Some(true),
        crlf: None,
    };
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "test");

    let result = translator_i.case_fold_char(span, '1');
}

