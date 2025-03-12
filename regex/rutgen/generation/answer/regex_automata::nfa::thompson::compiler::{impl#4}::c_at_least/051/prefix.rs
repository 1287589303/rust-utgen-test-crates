// Answer 0

#[test]
fn test_c_at_least_n_zero_cannot_match_empty_string() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    
    let expr = Hir::from_regex("a").unwrap(); // matches 'a', can't match empty string
    let result = compiler.c_at_least(&expr, true, 0);
}

#[test]
fn test_c_at_least_n_zero_can_match_empty_string() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    
    let expr = Hir::from_regex("(a)?").unwrap(); // can match empty string
    let result = compiler.c_at_least(&expr, true, 0);
}

#[test]
fn test_c_at_least_n_one() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = Hir::from_regex("a").unwrap(); // minimal expression
    let result = compiler.c_at_least(&expr, true, 1);
}

#[test]
fn test_c_at_least_n_greater_than_one() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let expr = Hir::from_regex("a").unwrap(); // minimal expression
    // Expecting to use c_exactly and get an error due to the context
    let result = compiler.c_at_least(&expr, true, 2);
}

