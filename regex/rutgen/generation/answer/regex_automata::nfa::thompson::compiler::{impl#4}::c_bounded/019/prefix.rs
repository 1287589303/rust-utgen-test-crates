// Answer 0

#[test]
fn test_c_bounded_greedy_min1_max2() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let expr = hir::Hir::concat(vec![
        hir::Hir::literal(b'a'),
        hir::Hir::literal(b'a'),
    ]);
    let result = compiler.c_bounded(&expr, true, 1, 3);
}

#[test]
fn test_c_bounded_not_greedy_min1_max3() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let expr = hir::Hir::literal(b'a');
    let result = compiler.c_bounded(&expr, false, 1, 4);
}

#[test]
fn test_c_bounded_greedy_min2_max4() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let expr = hir::Hir::repeat(Box::new(hir::Hir::literal(b'a')), 2, None);
    let result = compiler.c_bounded(&expr, true, 2, 5);
}

#[test]
fn test_c_bounded_not_greedy_min3_max5() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let expr = hir::Hir::capture(0, None, Box::new(hir::Hir::literal(b'a')));
    let result = compiler.c_bounded(&expr, false, 3, 6);
}

#[test]
fn test_c_bounded_greedy_min4_max5() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };
    let expr = hir::Hir::literal(b'b');
    let result = compiler.c_bounded(&expr, true, 4, 6);
}

