// Answer 0

#[test]
fn test_compile_with_limit_exprs_and_reverse_false() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            shrink: Some(true),
            which_captures: Some(WhichCaptures::All),
            look_matcher: Some(LookMatcher { lineterm: 0 }),
            unanchored_prefix: Some(false),
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 256,
            map: vec![],
        }),
    };
    
    let mut exprs = vec![];
    for _ in 0..PatternID::LIMIT {
        exprs.push(Arc::new(hir::Hir::Dot(hir::Dot::AnyByte)));
    }

    let result = compiler.compile(&exprs);
}

#[test]
fn test_compile_with_limit_exprs_and_nfa_size_limit() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            shrink: Some(true),
            which_captures: Some(WhichCaptures::All),
            look_matcher: Some(LookMatcher { lineterm: 0 }),
            unanchored_prefix: Some(false),
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 256,
            map: vec![],
        }),
    };
    
    let mut exprs = vec![];
    for _ in 0..PatternID::LIMIT {
        exprs.push(Arc::new(hir::Hir::Dot(hir::Dot::AnyByte)));
    }

    let _ = compiler.builder.borrow_mut().set_size_limit(Some(512));
    let result = compiler.compile(&exprs);
}

#[test]
fn test_compile_with_limit_exprs_and_no_unanchored_prefix() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            shrink: Some(true),
            which_captures: Some(WhichCaptures::All),
            look_matcher: Some(LookMatcher { lineterm: 0 }),
            unanchored_prefix: Some(false),
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 256,
            map: vec![],
        }),
    };

    let mut exprs = vec![];
    for _ in 0..PatternID::LIMIT {
        exprs.push(Arc::new(hir::Hir::Dot(hir::Dot::AnyByte)));
    }

    let result = compiler.compile(&exprs);
}

#[test]
#[should_panic]
fn test_compile_should_fail_due_to_at_least() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            shrink: Some(true),
            which_captures: Some(WhichCaptures::All),
            look_matcher: Some(LookMatcher { lineterm: 0 }),
            unanchored_prefix: Some(false),
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 256,
            map: vec![],
        }),
    };

    let mut exprs = vec![];
    for _ in 0..PatternID::LIMIT {
        exprs.push(Arc::new(hir::Hir::Dot(hir::Dot::AnyByte)));
    }

    let _ = compiler.c_at_least(&hir::Hir::Dot(hir::Dot::AnyByte), false, 0);
}

