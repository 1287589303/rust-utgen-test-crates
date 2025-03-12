// Answer 0

#[test]
fn test_c_unicode_class_ascii_empty() {
    let cls = hir::ClassUnicode::new_ascii(vec![]); // satisfies cls.is_ascii() true and cls.iter() is empty
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::new().utf8(true).reverse(false),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::new()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let _ = compiler.add_empty().unwrap(); // satisfies self.add_empty()? returning Ok
    let _ = compiler.c_unicode_class(&cls); // self.add_sparse(trans)? will return Err as there are no transitions
}

#[test]
#[should_panic]
fn test_c_unicode_class_ascii_fail() {
    let cls = hir::ClassUnicode::new_ascii(vec![]); // satisfies cls.is_ascii() true and cls.iter() is empty
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::new().utf8(true).reverse(false),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::new()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    compiler.add_empty().unwrap(); // satisfies self.add_empty()? returning Ok
    // This should fail because add_sparse(trans) will panic as trans is empty
    let _ = compiler.c_unicode_class(&cls);
}

