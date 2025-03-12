// Answer 0

#[test]
fn test_add_capture_end_with_zero_capture_index() {
    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let _ = compiler.add_capture_end(0);
}

#[test]
fn test_add_capture_end_with_valid_capture_index() {
    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    compiler.builder.borrow_mut().start_pattern().unwrap();
    let _ = compiler.add_capture_end(1);
}

#[test]
#[should_panic]
fn test_add_capture_end_with_out_of_bounds_capture_index() {
    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let _ = compiler.add_capture_end(10); // Assuming 10 is out of bounds
}

