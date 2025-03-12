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
    let result = compiler.c_look(&anchor);
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
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_startLF() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::StartLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_endLF() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::EndLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_startCRLF() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::StartCRLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_endCRLF() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::EndCRLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_wordAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordAsciiNegate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordUnicodeNegate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordStartAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordEndAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordStartUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordEndUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordStartHalfAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordEndHalfAscii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordStartHalfUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
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
fn test_c_look_wordEndHalfUnicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let anchor = hir::Look::WordEndHalfUnicode;
    let result = compiler.c_look(&anchor);
}

