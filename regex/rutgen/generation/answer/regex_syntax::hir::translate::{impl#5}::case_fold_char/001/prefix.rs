// Answer 0

#[test]
fn test_case_fold_char_with_non_mappable_unicode() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let trans = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { case_insensitive: Some(true), unicode: Some(true), ..Default::default() }), 
        utf8: false, 
        line_terminator: b'\n' 
    };
    let translator_i = TranslatorI::new(&trans, "©");
    let result = translator_i.case_fold_char(span, '©');
}

#[test]
fn test_case_fold_char_with_another_non_mappable_unicode() {
    let span = Span { start: Position::new(1), end: Position::new(2) };
    let trans = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { case_insensitive: Some(true), unicode: Some(true), ..Default::default() }), 
        utf8: false, 
        line_terminator: b'\n' 
    };
    let translator_i = TranslatorI::new(&trans, "µ");
    let result = translator_i.case_fold_char(span, 'µ');
}

#[test]
fn test_case_fold_char_with_symbol() {
    let span = Span { start: Position::new(2), end: Position::new(3) };
    let trans = Translator { 
        stack: RefCell::new(vec![]), 
        flags: Cell::new(Flags { case_insensitive: Some(true), unicode: Some(true), ..Default::default() }), 
        utf8: false, 
        line_terminator: b'\n' 
    };
    let translator_i = TranslatorI::new(&trans, "✓");
    let result = translator_i.case_fold_char(span, '✓');
}

