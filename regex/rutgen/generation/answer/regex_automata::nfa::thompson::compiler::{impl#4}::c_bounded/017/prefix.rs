// Answer 0

#[test]
fn test_c_bounded_case_1() {
    let expr = hir::Hir::new_literal(vec![b'a']);
    let greedy = false;
    let min = 1;
    let max = 5;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };

    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_2() {
    let expr = hir::Hir::new_class(hir::Class::Bytes(hir::ClassBytes::new(vec![b'b'])));
    let greedy = false;
    let min = 2;
    let max = 7;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };

    let result = compiler.c_bounded(&expr, greedy, min, max);
}

#[test]
fn test_c_bounded_case_3() {
    let expr = hir::Hir::new_repetition(Box::new(hir::Repetition::new(hir::Hir::new_literal(vec![b'c']), 1, 3)));
    let greedy = false;
    let min = 1;
    let max = 4;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 1, capacity: 10, map: vec![] }),
    };

    let result = compiler.c_bounded(&expr, greedy, min, max);
}

