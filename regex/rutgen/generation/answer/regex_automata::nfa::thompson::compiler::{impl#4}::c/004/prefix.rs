// Answer 0

#[test]
fn test_c_repetition_zero_or_one() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: Box::new(hir::Hir::from(hir::Literal::new(b'a'))),
    };
    
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_at_least() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Repetition {
        min: 1,
        max: None,
        greedy: true,
        sub: Box::new(hir::Hir::from(hir::Literal::new(b'b'))),
    };
    
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_exactly() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Repetition {
        min: 2,
        max: Some(2),
        greedy: true,
        sub: Box::new(hir::Hir::from(hir::Literal::new(b'c'))),
    };
    
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_repetition_bounded() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Repetition {
        min: 1,
        max: Some(3),
        greedy: false,
        sub: Box::new(hir::Hir::from(hir::Literal::new(b'd'))),
    };
    
    let _ = compiler.c(&expr);
}

