// Answer 0

#[test]
fn test_look_start() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::Start;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::End;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_startlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::StartLF;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_endlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::EndLF;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_startcrlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::StartCRLF;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_endcrlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::EndCRLF;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordAscii;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordasciinegate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordAsciiNegate;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordUnicode;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordunicodenegate() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordUnicodeNegate;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordstartascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordStartAscii;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordendascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordEndAscii;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordstartunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordStartUnicode;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordendunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordEndUnicode;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordstarthalfascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordStartHalfAscii;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordendhalfascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordEndHalfAscii;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordstarthalfunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordStartHalfUnicode;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

#[test]
fn test_look_wordendhalfunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = hir::Look::WordEndHalfUnicode;
    let hir_expr = Hir::from(look);
    let _ = compiler.c(&hir_expr);
}

