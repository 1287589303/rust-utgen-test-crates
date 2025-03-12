// Answer 0

#[test]
fn test_c_with_capture_valid_name() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Capture {
        index: 0,
        name: Some(Arc::from("capture_name")),
        sub: Box::new(hir::Hir::new_empty()),
    };

    let result = compiler.c(&expr);
}

#[test]
fn test_c_with_capture_empty_name() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Capture {
        index: 1,
        name: None,
        sub: Box::new(hir::Hir::new_empty()),
    };

    let result = compiler.c(&expr);
}

#[test]
fn test_c_with_capture_large_index() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = hir::Capture {
        index: 100,
        name: Some(Arc::from("capture_large_index")),
        sub: Box::new(hir::Hir::new_empty()),
    };

    let result = compiler.c(&expr);
}

