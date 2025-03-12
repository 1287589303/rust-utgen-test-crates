// Answer 0

#[test]
fn test_push_char_with_repeated_ascii() {
    let mut translator = Translator { 
        stack: RefCell::new(vec![HirFrame::Literal(vec![])]) ,
        flags: Cell::new(Flags::default()), 
        utf8: true,
        line_terminator: b'\n',
    };
    
    let translator_i = TranslatorI::new(&translator, "test");
    translator_i.push_char('a');
    translator_i.push_char('a');
}

#[test]
fn test_push_char_with_single_utf8() {
    let mut translator = Translator { 
        stack: RefCell::new(vec![HirFrame::Literal(vec![])]) ,
        flags: Cell::new(Flags::default()), 
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "test");
    translator_i.push_char('中');
}

#[test]
fn test_push_char_with_sequence() {
    let mut translator = Translator { 
        stack: RefCell::new(vec![HirFrame::Literal(vec![])]) ,
        flags: Cell::new(Flags::default()), 
        utf8: true,
        line_terminator: b'\n',
    };

    let translator_i = TranslatorI::new(&translator, "test");
    translator_i.push_char('a');
    translator_i.push_char('b');
    translator_i.push_char('中');
}

