// Answer 0

#[test]
fn test_c_bounded_case_1() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };
    let expr = hir::Hir::from(hir::Class::Bytes(hir::ClassBytes::new(vec![b'a'])));
    let greedy = true;
    let min = 1;
    let max = 2;
    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_2() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };
    let expr = hir::Hir::from(hir::Literal(hir::Literal::from_bytes(b"abc")));
    let greedy = false;
    let min = 3;
    let max = 4;
    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_3() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };
    let expr = hir::Hir::from(hir::Repetition::new(hir::Hir::from(hir::Class::Bytes(hir::ClassBytes::new(vec![b'b']))), 1..=2));
    let greedy = true;
    let min = 1;
    let max = 3;
    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_4() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };
    let expr = hir::Hir::from(hir::Literal(hir::Literal::from_bytes(b"def")));
    let greedy = false;
    let min = 5;
    let max = 6;
    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_5() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };
    let expr = hir::Hir::from(hir::Class::Unicode(hir::ClassUnicode::new(vec!['g'])));
    let greedy = true;
    let min = 1;
    let max = 10;
    let result = compiler.c_bounded(&expr, greedy, min, max);
}

