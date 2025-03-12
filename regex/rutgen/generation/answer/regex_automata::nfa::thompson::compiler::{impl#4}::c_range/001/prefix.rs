// Answer 0

#[test]
fn test_c_range_start_equals_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };

    let result = compiler.c_range(100, 100);
}

#[test]
fn test_c_range_start_greater_than_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };

    let result = compiler.c_range(200, 100);
}

#[test]
fn test_c_range_start_at_min_value() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };

    let result = compiler.c_range(0, 0);
}

#[test]
fn test_c_range_start_at_max_value() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };

    let result = compiler.c_range(255, 255);
}

