// Answer 0

#[test]
fn test_c_alt_iter_with_first_successful_second_error() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let first_result = Ok(ThompsonRef { start: StateID(0), end: StateID(1) });
    let second_result: Result<ThompsonRef, BuildError> = Err(BuildError { kind: BuildErrorKind::SomeKind });

    let iterator = vec![first_result, second_result].into_iter();

    let _ = compiler.c_alt_iter(iterator);
}

#[test]
fn test_c_alt_iter_with_first_successful_second_successful() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let first_result = Ok(ThompsonRef { start: StateID(0), end: StateID(1) });
    let second_result = Ok(ThompsonRef { start: StateID(2), end: StateID(3) });
    let additional_result = Ok(ThompsonRef { start: StateID(4), end: StateID(5) });

    let iterator = vec![first_result, second_result, additional_result].into_iter();

    let _ = compiler.c_alt_iter(iterator);
}

#[test]
#[should_panic]
fn test_c_alt_iter_with_empty_iterator() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let iterator: Vec<Result<ThompsonRef, BuildError>> = Vec::new();

    let _ = compiler.c_alt_iter(iterator.into_iter());
}

