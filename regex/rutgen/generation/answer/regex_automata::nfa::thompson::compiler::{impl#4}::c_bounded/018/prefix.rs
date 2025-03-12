// Answer 0

#[test]
fn test_c_bounded_min_less_than_max() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, HirKind};

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 10, map: Vec::new() }),
    };

    let expr: Hir = Class(ClassBytes::new(b"a".to_vec()).into()).into();
    let min = 1;
    let max = 2;

    let _ = compiler.c_bounded(&expr, true, min, max);
}

#[test]
#[should_panic]
fn test_c_bounded_patch_should_fail() {
    use regex_syntax::hir::{Hir, Class, ClassBytes, HirKind};

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 10, map: Vec::new() }),
    };

    let expr: Hir = Class(ClassBytes::new(b"a".to_vec()).into()).into();
    let min = 1;
    let max = 3;

    let _ = compiler.c_bounded(&expr, true, min, max);
}

