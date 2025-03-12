// Answer 0

#[test]
fn test_peek_space_with_non_whitespace_characters() {
    struct TestParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0 }),
                ignore_whitespace: Cell::new(true),
                pattern: pattern.to_string(),
            }
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            let start = self.offset() + 1;
            self.pattern.chars().nth(start)
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }

        fn char_at(&self, i: usize) -> char {
            self.pattern.chars().nth(i).unwrap_or('\0')
        }
    }

    let parser = TestParser::new("abc # comment");
    let result = parser.peek(); // Simulates invoking peek_space
}

#[test]
fn test_peek_space_with_comments() {
    struct TestParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0 }),
                ignore_whitespace: Cell::new(true),
                pattern: pattern.to_string(),
            }
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            let start = self.offset() + 1;
            self.pattern.chars().nth(start)
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }

        fn char_at(&self, i: usize) -> char {
            self.pattern.chars().nth(i).unwrap_or('\0')
        }
    }

    let parser = TestParser::new("xyz # this is a comment");
    let result = parser.peek(); // Simulates invoking peek_space
}

