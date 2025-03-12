// Answer 0

#[test]
fn test_push_char_appends_to_existing_literal() {
    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Literal(vec![b'a'])]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.push_char('b');
}

#[test]
fn test_push_char_appends_accented_character() {
    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Literal(vec![b'e', b'x'])]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.push_char('é');
}

#[test]
fn test_push_char_appends_symbol() {
    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Literal(vec![b'1'])]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.push_char('#');
}

#[test]
fn test_push_char_appends_surrogate_pair_character() {
    let translator = Translator {
        stack: RefCell::new(vec![HirFrame::Literal(vec![b'c'])]),
        flags: Cell::new(Flags::default()),
        utf8: true,
        line_terminator: b'\n',
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.push_char('𐍈'); // U+10348 Gothic letter hwair
}

