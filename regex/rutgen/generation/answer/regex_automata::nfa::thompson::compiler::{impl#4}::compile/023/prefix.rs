// Answer 0

#[test]
fn test_compile_with_maximum_patterns() {
    use regex_syntax::hir::{Hir, ClassBytes};
    use regex_syntax::ParserBuilder;

    let mut builder = Builder::new();
    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::new()
            .utf8(true)
            .reverse(true)
            .nfa_size_limit(Some(1024))
            .which_captures(WhichCaptures::None)
            .build(),
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let patterns = vec![Hir::class_bytes(ClassBytes::new(b"abc")), 
                        Hir::class_bytes(ClassBytes::new(b"def"))];

    let exprs: Vec<&Hir> = patterns.iter().collect();

    // Set up the builder size limit and unanchored prefix
    compiler.builder.borrow_mut().set_size_limit(Some(2048)).unwrap();
    compiler.config.unanchored_prefix = Some(false);

    // Compile the expressions
    let result = compiler.compile(&exprs);

    // Since we don’t validate outcomes, the test passes if no panic occurs
    let _ = result.ok();
}

#[test]
fn test_compile_with_pattern_limit() {
    use regex_syntax::hir::{Hir, ClassBytes};
    use regex_syntax::ParserBuilder;

    let max_patterns = PatternID::LIMIT;
    let mut builder = Builder::new();
    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::new()
            .utf8(true)
            .reverse(true)
            .nfa_size_limit(Some(1024))
            .which_captures(WhichCaptures::None)
            .build(),
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let patterns: Vec<Hir> = (0..max_patterns).map(|i| Hir::class_bytes(ClassBytes::new(format!("pattern_{}", i).as_bytes()))).collect();
    let exprs: Vec<&Hir> = patterns.iter().collect();

    compiler.builder.borrow_mut().set_size_limit(Some(2048)).unwrap();
    compiler.config.unanchored_prefix = Some(false);

    let result = compiler.compile(&exprs);

    let _ = result.ok();
}

