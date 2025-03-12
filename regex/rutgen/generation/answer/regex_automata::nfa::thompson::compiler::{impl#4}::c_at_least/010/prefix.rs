// Answer 0

#[test]
fn test_c_at_least_with_non_zero_n_minimum_length_greedy_false() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 1,
            capacity: 10,
            map: vec![],
        }),
    };
    
    let expr = Hir::from_literal(b"test");
    let greedy = false;
    let n = 1;

    let union = compiler.add_union_reverse().unwrap();
    let compiled = compiler.c(&expr).unwrap();
    compiler.patch(union, compiled.start).unwrap();
    compiler.patch(compiled.end, union).unwrap();

    let result = compiler.c_at_least(&expr, greedy, n);
}

#[test]
fn test_c_at_least_with_non_zero_n_minimum_length_greedy_false_case2() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 1,
            capacity: 10,
            map: vec![],
        }),
    };

    let expr = Hir::from_literal(b"example");
    let greedy = false;
    let n = 2;

    let union = compiler.add_union_reverse().unwrap();
    let compiled = compiler.c(&expr).unwrap();
    compiler.patch(union, compiled.start).unwrap();
    compiler.patch(compiled.end, union).unwrap();

    let result = compiler.c_at_least(&expr, greedy, n);
}

