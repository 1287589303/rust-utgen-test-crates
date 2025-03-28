// Answer 0

#[test]
fn test_maybe_parse_posix_class_invalid_name() {
    struct TestParser<'a> {
        config: Config,
        pattern: &'a str,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl<'a> TestParser<'a> {
        fn new(pattern: &'a str) -> Self {
            TestParser {
                config: Config { nest_limit: 10, flags: Flags::default() },
                pattern,
                depth: Cell::new(0),
                pos: Cell::new(0),
                char: Cell::new(Some(pattern.chars().next().unwrap())),
                capture_index: Cell::new(0),
                flags: RefCell::new(Flags::default()),
                capture_names: RefCell::new(vec![]),
            }
        }

        fn bump(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            self.char.set(self.pattern.chars().nth(self.pos.get()).or(None));
            self.char.is_some()
        }

        fn is_done(&self) -> bool {
            self.pos.get() >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.char.get().unwrap()
        }

        fn bump_if(&self, chars: &str) -> bool {
            if chars.contains(self.char()) {
                self.bump();
                true
            } else {
                false
            }
        }

        fn maybe_parse_posix_class(&self) -> Option<Class> {
            assert_eq!(self.char(), '[');
            let start_pos = self.pos();
            let start_char = self.char.get();
            let reset = || {
                self.pos.set(start_pos);
                self.char.set(start_char);
            };

            let mut negated = false;
            if !self.bump() || self.char() != ':' {
                reset();
                return None;
            }
            if !self.bump() {
                reset();
                return None;
            }
            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    reset();
                    return None;
                }
            }
            let name_start = self.pos();
            while self.char() != ':' && self.bump() {}
            if self.is_done() {
                reset();
                return None;
            }
            let name = &self.pattern[name_start..self.pos()];
            if !self.bump_if(":]") {
                reset();
                return None;
            }
            if let Ok(ranges) = posix_class(name) {
                let mut class = Class::new(ranges);
                if negated {
                    class.negate();
                }
                return Some(class);
            }
            reset();
            None
        }
    }

    let parser = TestParser::new("[[:loower:]]");
    let result = parser.maybe_parse_posix_class();
}

