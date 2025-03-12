// Answer 0

#[test]
fn test_compile_with_limit_patterns() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            ..Config::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let patterns: Vec<Hir> = (0..PatternID::LIMIT).map(|_| Hir::empty()).collect();
    let exprs: Vec<&Hir> = patterns.iter().collect();

    let _ = compiler.compile(&exprs);
} 

#[test]
fn test_compile_with_unanchored_prefix_false() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            unanchored_prefix: Some(false),
            ..Config::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let patterns: Vec<Hir> = (0..PatternID::LIMIT).map(|_| Hir::empty()).collect();
    let exprs: Vec<&Hir> = patterns.iter().collect();

    let _ = compiler.compile(&exprs);
}

#[test]
fn test_compile_with_patch_err() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            utf8: Some(true),
            reverse: Some(false),
            nfa_size_limit: Some(Some(1024)),
            unanchored_prefix: Some(false),
            ..Config::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let patterns: Vec<Hir> = (0..PatternID::LIMIT).map(|_| Hir::empty()).collect();
    let exprs: Vec<&Hir> = patterns.iter().collect();

    let unanchored_prefix = ThompsonRef {
        start: StateID::default(),
        end: StateID::default(),
    };

    compiler.builder.borrow_mut().set_size_limit(Some(128)).unwrap();
    compiler.c_at_least(&Hir::dot(hir::Dot::AnyByte), false, 0).unwrap();
    
    let compiled_result = compiler.c_alt_iter(exprs.iter().map(|e| {
        let _ = compiler.start_pattern().unwrap();
        let one = compiler.c_cap(0, None, e.borrow()).unwrap();
        let match_state_id = compiler.add_match().unwrap();
        compiler.patch(one.end, match_state_id).unwrap();
        let _ = compiler.finish_pattern(one.start).unwrap();
        Ok(ThompsonRef { start: one.start, end: match_state_id })
    }));

    let _ = compiled_result.expect_err("Expected patch to return an error");
}

