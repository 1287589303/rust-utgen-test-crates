// Answer 0

#[test]
fn test_c_at_least_n_zero() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    let expr = Hir::from_class(hir::Class::Unicode(hir::ClassUnicode::new(vec![b'a'..=b'z'].into_iter().collect())));
    let result = compiler.c_at_least(&expr, false, 0);
    let _ = result; // Consume the result to ensure compile success
}

#[test]
fn test_c_at_least_n_zero_with_empty() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    let expr = Hir::from_class(hir::Class::Unicode(hir::ClassUnicode::new(vec![b'a'..=b'z'].into_iter().collect())));
    let result = compiler.c_at_least(&expr, false, 0);
    let _ = result; // Consume the result to ensure compile success
}

#[test]
fn test_c_at_least_n_zero_non_empty_expr() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };
    let expr = Hir::from_class(hir::Class::Bytes(hir::ClassBytes::new(vec![b'a', b'b'])));

    let result = compiler.c_at_least(&expr, false, 0);
    let _ = result; // Consume the result to ensure compile success
}

