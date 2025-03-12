// Answer 0

#[test]
fn test_c_alt_iter_success() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let first_ref = ThompsonRef {
        start: StateID(0),
        end: StateID(1),
    };

    let second_ref = ThompsonRef {
        start: StateID(2),
        end: StateID(3),
    };

    let iter = vec![
        Ok(first_ref.clone()),
        Ok(second_ref.clone()),
        Ok(ThompsonRef {
            start: StateID(4),
            end: StateID(5),
        }),
    ].into_iter();

    let result = compiler.c_alt_iter(iter);
}

#[test]
fn test_c_alt_iter_patch_error() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let first_ref = ThompsonRef {
        start: StateID(0),
        end: StateID(1),
    };

    let second_ref = ThompsonRef {
        start: StateID(2),
        end: StateID(3),
    };

    let iter = vec![
        Ok(first_ref.clone()),
        Ok(second_ref.clone()),
        Err(BuildError { kind: BuildErrorKind::SomeError }),
    ].into_iter();

    let result = compiler.c_alt_iter(iter);
}

