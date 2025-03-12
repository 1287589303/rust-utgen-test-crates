// Answer 0

#[test]
fn test_c_look_start() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
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
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::End;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_start_lf() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::StartLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_end_lf() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::EndLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_start_crlf() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::StartCRLF;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_end_crlf() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::EndCRLF;
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
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_ascii_negate() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordAsciiNegate;
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
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordUnicode;
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
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordUnicodeNegate;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_ascii() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordStartAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_ascii() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordEndAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_unicode() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordStartUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_unicode() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordEndUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_half_ascii() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordStartHalfAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_half_ascii() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordEndHalfAscii;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_start_half_unicode() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordStartHalfUnicode;
    let result = compiler.c_look(&anchor);
}

#[test]
fn test_c_look_word_end_half_unicode() {
    let compiler = Compiler { 
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };
    let anchor = hir::Look::WordEndHalfUnicode;
    let result = compiler.c_look(&anchor);
}

