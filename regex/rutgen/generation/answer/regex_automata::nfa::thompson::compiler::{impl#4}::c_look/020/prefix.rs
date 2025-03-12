// Answer 0

#[test]
fn test_c_look_word_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordUnicode;
    let _ = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_unicode_negate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordUnicodeNegate;
    let _ = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordStartAscii;
    let _ = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordEndAscii;
    let _ = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordStartUnicode;
    let _ = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let anchor = hir::Look::WordEndUnicode;
    let _ = compiler.c_look(&anchor);
}

