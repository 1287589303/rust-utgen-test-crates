// Answer 0

#[test]
fn test_add_look_reverse_start() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::Start;
    let _ = compiler.add_look(look);
}

#[test]
fn test_add_look_reverse_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::End;
    let _ = compiler.add_look(look);
}

#[test]
fn test_add_look_reverse_start_lf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::StartLF;
    let _ = compiler.add_look(look);
}

#[test]
fn test_add_look_reverse_end_lf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::EndLF;
    let _ = compiler.add_look(look);
}

#[test]
fn test_add_look_reverse_word_ascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::WordAscii;
    let _ = compiler.add_look(look);
}

#[test]
fn test_add_look_reverse_word_unicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: Some(true),
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };
    let look = Look::WordUnicode;
    let _ = compiler.add_look(look);
}

