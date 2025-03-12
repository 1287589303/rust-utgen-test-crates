// Answer 0

#[test]
fn test_c_alt_iter_with_two_results() {
    struct MockHir;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let result1 = Ok(ThompsonRef { start: StateID(1), end: StateID(2) });
    let result2 = Ok(ThompsonRef { start: StateID(3), end: StateID(4) });

    let input_iterator = vec![result1, result2].into_iter();

    let _ = compiler.c_alt_iter(input_iterator);
}

#[test]
fn test_c_alt_iter_with_result_and_end() {
    struct MockHir;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let result1 = Ok(ThompsonRef { start: StateID(5), end: StateID(6) });
    let result2 = Ok(ThompsonRef { start: StateID(7), end: StateID(8) });

    let input_iterator = vec![result1, result2].into_iter();

    let _ = compiler.c_alt_iter(input_iterator);
}

#[test]
fn test_c_alt_iter_with_prepped_union_and_empty() {
    struct MockHir;

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let result1 = Ok(ThompsonRef { start: StateID(9), end: StateID(10) });
    let result2 = Ok(ThompsonRef { start: StateID(11), end: StateID(12) });

    let input_iterator = vec![result1, result2].into_iter();

    let _ = compiler.c_alt_iter(input_iterator);
}

