// Answer 0

#[test]
fn test_case_fold_char_uppercase_a() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "A");
    translator_i.case_fold_char(span, 'A');
}

#[test]
fn test_case_fold_char_uppercase_b() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "B");
    translator_i.case_fold_char(span, 'B');
}

#[test]
fn test_case_fold_char_uppercase_c() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "C");
    translator_i.case_fold_char(span, 'C');
}

#[test]
fn test_case_fold_char_lowercase_a() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "a");
    translator_i.case_fold_char(span, 'a');
}

#[test]
fn test_case_fold_char_lowercase_b() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "b");
    translator_i.case_fold_char(span, 'b');
}

#[test]
fn test_case_fold_char_lowercase_c() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut flags = Flags { case_insensitive: Some(true), unicode: Some(false), ..Default::default() };
    let translator = Translator { flags: Cell::new(flags), ..Default::default() };
    let translator_i = TranslatorI::new(&translator, "c");
    translator_i.case_fold_char(span, 'c');
}

