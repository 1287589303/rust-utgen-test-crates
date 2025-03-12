// Answer 0

#[test]
fn test_c_look_word_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_ascii_negate() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordAsciiNegate;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_unicode_negate() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordUnicodeNegate;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_half_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartHalfAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_half_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndHalfAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_half_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartHalfUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_half_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndHalfUnicode;
    let result = compiler.c_look(&anchor);
}

