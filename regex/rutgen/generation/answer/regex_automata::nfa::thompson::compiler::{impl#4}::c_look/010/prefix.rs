// Answer 0

#[test]
fn test_c_look_start() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::Start;
    let _ = compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::End;
    let _ = compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_word_end_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndUnicode;
    let _ = compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_word_start_half_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartHalfUnicode;
    let _ = compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_word_end_half_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndHalfAscii;
    let _ = compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_word_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordAscii;
    let _ = compiler.c_look(&anchor).unwrap();
}

