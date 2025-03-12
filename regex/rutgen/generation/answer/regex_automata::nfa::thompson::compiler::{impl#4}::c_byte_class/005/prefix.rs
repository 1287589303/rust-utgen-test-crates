// Answer 0

#[test]
fn test_c_byte_class_empty_class() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
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

    let class_bytes = hir::ClassBytes::new(vec![]);

    let result = compiler.c_byte_class(&class_bytes);
}

#[test]
fn test_c_byte_class_single_range() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
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

    let class_bytes = hir::ClassBytes::new(vec![(b'a', b'a')]);

    let result = compiler.c_byte_class(&class_bytes);
} 

#[test]
fn test_c_byte_class_multiple_ranges() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
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

    let class_bytes = hir::ClassBytes::new(vec![
        (b'a', b'a'),
        (b'b', b'b'),
        (b'c', b'c'),
    ]);

    let result = compiler.c_byte_class(&class_bytes);
}

