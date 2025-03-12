// Answer 0

#[test]
fn test_c_look_start() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::Start;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_end() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::End;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_startLF() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::StartLF;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_endLF() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::EndLF;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_wordAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordAscii;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_wordUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordUnicode;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_wordStartHalfAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordStartHalfAscii;
    compiler.c_look(&anchor).unwrap();
}

#[test]
fn test_c_look_wordEndHalfUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndHalfUnicode;
    compiler.c_look(&anchor).unwrap();
}

