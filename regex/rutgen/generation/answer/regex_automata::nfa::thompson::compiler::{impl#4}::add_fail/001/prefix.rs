// Answer 0

#[test]
fn test_add_fail_with_initialized_builder() {
    let builder = Builder::new();
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 10,
            map: vec![],
        }),
    };
    let _ = compiler.add_fail();
}

#[test]
#[should_panic]
fn test_add_fail_with_empty_builder() {
    let builder = Builder::new();
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 10,
            map: vec![],
        }),
    };
    let _ = compiler.add_fail();
}

