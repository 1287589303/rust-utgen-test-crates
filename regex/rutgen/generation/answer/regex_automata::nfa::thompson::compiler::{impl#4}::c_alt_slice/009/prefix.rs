// Answer 0

#[test]
fn test_c_alt_slice_with_literals() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { reverse: Some(false), ..Default::default() },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let exprs = vec![
        hir::Hir::literal(hir::Literal::new(b"test1")),
        hir::Hir::literal(hir::Literal::new(b"test2")),
    ];

    let _ = compiler.c_alt_slice(&exprs);
}

#[test]
fn test_c_alt_slice_with_same_literals() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { reverse: Some(false), ..Default::default() },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let exprs = vec![
        hir::Hir::literal(hir::Literal::new(b"same")),
        hir::Hir::literal(hir::Literal::new(b"same")),
    ];

    let _ = compiler.c_alt_slice(&exprs);
}

#[test]
fn test_c_alt_slice_with_multiple_literals() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { reverse: Some(false), ..Default::default() },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let exprs = vec![
        hir::Hir::literal(hir::Literal::new(b"literal1")),
        hir::Hir::literal(hir::Literal::new(b"literal2")),
        hir::Hir::literal(hir::Literal::new(b"literal3")),
    ];

    let _ = compiler.c_alt_slice(&exprs);
}

#[test]
fn test_c_alt_slice_with_reverse_disabled() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { reverse: Some(false), ..Default::default() },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let exprs = vec![
        hir::Hir::literal(hir::Literal::new(b"first")),
        hir::Hir::literal(hir::Literal::new(b"second")),
    ];

    let _ = compiler.c_alt_slice(&exprs);
}

