// Answer 0

#[test]
fn test_c_unicode_class_non_ascii_reverse_no_shrink() {
    let mut builder = Builder::default();
    let mut utf8_state = Utf8State::default();
    let config = Config {
        utf8: Some(true),
        reverse: Some(true),
        shrink: Some(false),
        ..Default::default()
    };
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config,
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(utf8_state),
        trie_state: RefCell::new(RangeTrie::new()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let cls = hir::ClassUnicode::new(vec![
        hir::Range::new_unchecked(0x80, 0x90), // Non-ASCII range
    ]);

    let _result = compiler.c_unicode_class(&cls);
}

#[test]
fn test_c_unicode_class_non_ascii_reverse_no_shrink_empty() {
    let mut builder = Builder::default();
    let mut utf8_state = Utf8State::default();
    let config = Config {
        utf8: Some(true),
        reverse: Some(true),
        shrink: Some(false),
        ..Default::default()
    };
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config,
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(utf8_state),
        trie_state: RefCell::new(RangeTrie::new()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let cls = hir::ClassUnicode::new(vec![]); // Empty class

    let _result = compiler.c_unicode_class(&cls);
}

#[test]
fn test_c_unicode_class_non_ascii_reverse_no_shrink_multiple_ranges() {
    let mut builder = Builder::default();
    let mut utf8_state = Utf8State::default();
    let config = Config {
        utf8: Some(true),
        reverse: Some(true),
        shrink: Some(false),
        ..Default::default()
    };
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config,
        builder: RefCell::new(builder),
        utf8_state: RefCell::new(utf8_state),
        trie_state: RefCell::new(RangeTrie::new()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let cls = hir::ClassUnicode::new(vec![
        hir::Range::new_unchecked(0x80, 0x90),
        hir::Range::new_unchecked(0xA0, 0xB0), // Another non-ASCII range
    ]);

    let _result = compiler.c_unicode_class(&cls);
}

