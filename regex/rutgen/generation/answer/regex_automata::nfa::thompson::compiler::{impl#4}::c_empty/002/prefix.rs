// Answer 0

#[test]
fn test_c_empty_success() {
    let builder = Builder {
        config: Config::default(),
        ..Default::default()
    };
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    let result = compiler.c_empty();
}

