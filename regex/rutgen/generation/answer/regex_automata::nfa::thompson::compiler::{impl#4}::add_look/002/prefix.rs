// Answer 0

#[test]
fn test_add_look_start() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::Start);
}

#[test]
fn test_add_look_end() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::End);
}

#[test]
fn test_add_look_startlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::StartLF);
}

#[test]
fn test_add_look_endlf() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::EndLF);
}

#[test]
fn test_add_look_wordascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordAscii);
}

#[test]
fn test_add_look_wordunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordUnicode);
}

#[test]
fn test_add_look_wordstartascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordStartAscii);
}

#[test]
fn test_add_look_wordendascii() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordEndAscii);
}

#[test]
fn test_add_look_wordstartunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordStartUnicode);
}

#[test]
fn test_add_look_wordendunicode() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            reverse: false,
            ..Default::default()
        },
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };

    let _ = compiler.add_look(Look::WordEndUnicode);
}

