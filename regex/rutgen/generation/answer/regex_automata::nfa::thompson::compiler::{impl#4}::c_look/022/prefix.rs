// Answer 0

#[test]
fn test_c_look_word_ascii_negate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    
    let anchor = hir::Look::WordAsciiNegate;

    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let anchor = hir::Look::WordAscii;

    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_unicode_negate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let anchor = hir::Look::WordUnicodeNegate;

    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let anchor = hir::Look::WordUnicode;

    let result = compiler.c_look(&anchor);
}

