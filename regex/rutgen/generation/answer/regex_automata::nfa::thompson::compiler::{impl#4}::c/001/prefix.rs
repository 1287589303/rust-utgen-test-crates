// Answer 0

#[test]
fn test_c_with_alternation_of_literals() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    
    let exprs = vec![
        hir::Hir::from(hir::Literal(hir::Literal::from(b"abc"))),
        hir::Hir::from(hir::Literal(hir::Literal::from(b"def"))),
    ];
    
    let result = compiler.c(&hir::Hir::from(hir::Alternation(exprs)));
}

#[test]
fn test_c_with_alternation_of_classes() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let exprs = vec![
        hir::Hir::from(hir::Class::Bytes(hir::ClassBytes::new(vec![(b'a', b'z')]))),
        hir::Hir::from(hir::Class::Unicode(hir::ClassUnicode::new(vec![(1, 10)]))),
    ];
    
    let result = compiler.c(&hir::Hir::from(hir::Alternation(exprs)));
}

#[test]
fn test_c_with_mixed_alternation_types() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    
    let exprs = vec![
        hir::Hir::from(hir::Literal(hir::Literal::from(b"foo"))),
        hir::Hir::from(hir::Class::Bytes(hir::ClassBytes::new(vec![(b'0', b'9')]))),
        hir::Hir::from(hir::Look::Start),
        hir::Hir::from(hir::Repetition::new(hir::Hir::from(hir::Literal(hir::Literal::from(b"bar"))), 0, None)),
    ];
    
    let result = compiler.c(&hir::Hir::from(hir::Alternation(exprs)));
}

