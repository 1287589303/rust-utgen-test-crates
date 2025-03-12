// Answer 0

#[test]
fn test_c_repetition_zero_or_one() {
    let rep = hir::Repetition { min: 0, max: Some(1), greedy: true, sub: hir::Hir::Empty };
    let compiler = Compiler { 
        parser: ParserBuilder::default(), 
        config: Config::default(), 
        builder: RefCell::new(Builder::default()), 
        utf8_state: RefCell::new(Utf8State::default()), 
        trie_state: RefCell::new(RangeTrie::default()), 
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()) 
    };
    let _ = compiler.c_repetition(&rep);
}

#[test]
fn test_c_repetition_at_least() {
    let rep = hir::Repetition { min: 1, max: None, greedy: false, sub: hir::Hir::Empty };
    let compiler = Compiler { 
        parser: ParserBuilder::default(), 
        config: Config::default(), 
        builder: RefCell::new(Builder::default()), 
        utf8_state: RefCell::new(Utf8State::default()), 
        trie_state: RefCell::new(RangeTrie::default()), 
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()) 
    };
    let _ = compiler.c_repetition(&rep);
}

#[test]
fn test_c_repetition_exactly() {
    let rep = hir::Repetition { min: 2, max: Some(2), greedy: true, sub: hir::Hir::Empty };
    let compiler = Compiler { 
        parser: ParserBuilder::default(), 
        config: Config::default(), 
        builder: RefCell::new(Builder::default()), 
        utf8_state: RefCell::new(Utf8State::default()), 
        trie_state: RefCell::new(RangeTrie::default()), 
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()) 
    };
    let _ = compiler.c_repetition(&rep);
}

#[test]
fn test_c_repetition_bounded() {
    let rep = hir::Repetition { min: 2, max: Some(5), greedy: false, sub: hir::Hir::Empty };
    let compiler = Compiler { 
        parser: ParserBuilder::default(), 
        config: Config::default(), 
        builder: RefCell::new(Builder::default()), 
        utf8_state: RefCell::new(Utf8State::default()), 
        trie_state: RefCell::new(RangeTrie::default()), 
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()) 
    };
    let _ = compiler.c_repetition(&rep);
}

