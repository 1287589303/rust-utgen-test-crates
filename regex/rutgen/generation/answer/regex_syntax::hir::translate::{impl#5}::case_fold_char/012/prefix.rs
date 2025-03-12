// Answer 0

#[test]
fn test_case_fold_char_non_ascii_1() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let span = Span { start: 0, end: 1 };
    let result = translator.case_fold_char(span, 'ç');
}

#[test]
fn test_case_fold_char_non_ascii_2() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let span = Span { start: 0, end: 1 };
    let result = translator.case_fold_char(span, 'ë');
}

#[test]
fn test_case_fold_char_non_ascii_3() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let span = Span { start: 0, end: 1 };
    let result = translator.case_fold_char(span, 'ö');
}

#[test]
fn test_case_fold_char_non_ascii_4() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let span = Span { start: 0, end: 1 };
    let result = translator.case_fold_char(span, '的');
}

#[test]
fn test_case_fold_char_non_ascii_5() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: true,
        line_terminator: b'\n',
    };
    let span = Span { start: 0, end: 1 };
    let result = translator.case_fold_char(span, '😊');
}

