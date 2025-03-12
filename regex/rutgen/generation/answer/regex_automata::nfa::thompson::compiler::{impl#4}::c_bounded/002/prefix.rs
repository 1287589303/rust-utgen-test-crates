// Answer 0

#[test]
fn test_c_bounded_greedy_equal_min_max_1() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"a".to_vec());
    let result = compiler.c_bounded(&expr, true, 1, 1);
}

#[test]
fn test_c_bounded_non_greedy_equal_min_max_2() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"b".to_vec());
    let result = compiler.c_bounded(&expr, false, 2, 2);
}

#[test]
fn test_c_bounded_greedy_equal_min_max_3() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"c".to_vec());
    let result = compiler.c_bounded(&expr, true, 3, 3);
}

#[test]
fn test_c_bounded_non_greedy_equal_min_max_4() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"d".to_vec());
    let result = compiler.c_bounded(&expr, false, 4, 4);
}

#[test]
fn test_c_bounded_greedy_equal_min_max_5() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"e".to_vec());
    let result = compiler.c_bounded(&expr, true, 5, 5);
}

#[test]
fn test_c_bounded_non_greedy_equal_min_max_6() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"f".to_vec());
    let result = compiler.c_bounded(&expr, false, 6, 6);
}

#[test]
fn test_c_bounded_greedy_equal_min_max_7() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"g".to_vec());
    let result = compiler.c_bounded(&expr, true, 7, 7);
}

#[test]
fn test_c_bounded_non_greedy_equal_min_max_8() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"h".to_vec());
    let result = compiler.c_bounded(&expr, false, 8, 8);
}

#[test]
fn test_c_bounded_greedy_equal_min_max_9() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"i".to_vec());
    let result = compiler.c_bounded(&expr, true, 9, 9);
}

#[test]
fn test_c_bounded_non_greedy_equal_min_max_10() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = hir::Hir::literal(b"j".to_vec());
    let result = compiler.c_bounded(&expr, false, 10, 10);
}

