// Answer 0

#[test]
fn test_compile_empty_literal() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = Hir::empty();
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_single_byte_literal() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = Hir::literal(vec![1]);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_multi_byte_literal() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let expr = Hir::literal(vec![1, 2, 3]);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_class() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let unicode_class = hir::ClassUnicode::new(vec![(0x20, 0x7E)]);
    let expr = Hir::class_unicode(unicode_class);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_byte_class() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let byte_class = hir::ClassBytes::new(vec![(0, 255)]);
    let expr = Hir::class_bytes(byte_class);
    let _ = compiler.c(&expr);
}

