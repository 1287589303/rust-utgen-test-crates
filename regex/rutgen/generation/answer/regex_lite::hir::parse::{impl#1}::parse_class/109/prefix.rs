// Answer 0

#[test]
fn test_parse_class_valid() {
    struct TestParser {
        config: Config,
        pattern: String,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                config: Config {
                    size_limit: None,
                },
                pattern: pattern.to_string(),
                depth: Cell::new(0),
                pos: Cell::new(0),
                char: Cell::new(Some('[')),
                capture_index: Cell::new(0),
                flags: RefCell::new(Flags {
                    case_insensitive: false,
                    multi_line: false,
                    dot_matches_new_line: false,
                    swap_greed: false,
                    crlf: false,
                    ignore_whitespace: false,
                }),
                capture_names: RefCell::new(vec![]),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            true
        }

        fn char(&self) -> char {
            self.char.get().unwrap_or(' ')
        }

        fn is_done(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }

        fn bump(&self) {
            self.pos.set(self.pos.get() + 1);
        }
    }

    let parser = TestParser::new("[-]");
    let result = parser.parse_class();
}  

#[test]
fn test_parse_class_with_multiple_dashes() {
    struct TestParser {
        config: Config,
        pattern: String,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                config: Config {
                    size_limit: None,
                },
                pattern: pattern.to_string(),
                depth: Cell::new(0),
                pos: Cell::new(0),
                char: Cell::new(Some('[')),
                capture_index: Cell::new(0),
                flags: RefCell::new(Flags {
                    case_insensitive: false,
                    multi_line: false,
                    dot_matches_new_line: false,
                    swap_greed: false,
                    crlf: false,
                    ignore_whitespace: false,
                }),
                capture_names: RefCell::new(vec![]),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            true
        }

        fn char(&self) -> char {
            self.char.get().unwrap_or(' ')
        }

        fn is_done(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }

        fn bump(&self) {
            self.pos.set(self.pos.get() + 1);
        }
    }

    let parser = TestParser::new("[---]");
    let result = parser.parse_class();
}  

#[test]
fn test_parse_class_with_empty_content() {
    struct TestParser {
        config: Config,
        pattern: String,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                config: Config {
                    size_limit: None,
                },
                pattern: pattern.to_string(),
                depth: Cell::new(0),
                pos: Cell::new(0),
                char: Cell::new(Some('[')),
                capture_index: Cell::new(0),
                flags: RefCell::new(Flags {
                    case_insensitive: false,
                    multi_line: false,
                    dot_matches_new_line: false,
                    swap_greed: false,
                    crlf: false,
                    ignore_whitespace: false,
                }),
                capture_names: RefCell::new(vec![]),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            true
        }

        fn char(&self) -> char {
            self.char.get().unwrap_or(' ')
        }

        fn is_done(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }

        fn bump(&self) {
            self.pos.set(self.pos.get() + 1);
        }
    }

    let parser = TestParser::new("[ ]");
    let result = parser.parse_class();
}  

