// Answer 0

#[test]
fn test_peek_space_with_comment() {
    struct MockParser {
        ignore_whitespace: Cell<bool>,
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(ignore_whitespace: bool, offset: usize, pattern: &str) -> Self {
            MockParser {
                ignore_whitespace: Cell::new(ignore_whitespace),
                pos: Cell::new(Position { offset: offset }),
                pattern: pattern.to_string(),
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap_or('\0')
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = MockParser::new(true, 8, "abc # this is a comment\n def");
    let parser_instance = ParserI::new(&parser, parser.pattern());

    let result = parser_instance.peek_space();
}

#[test]
fn test_peek_space_without_comment() {
    struct MockParser {
        ignore_whitespace: Cell<bool>,
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(ignore_whitespace: bool, offset: usize, pattern: &str) -> Self {
            MockParser {
                ignore_whitespace: Cell::new(ignore_whitespace),
                pos: Cell::new(Position { offset: offset }),
                pattern: pattern.to_string(),
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap_or('\0')
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = MockParser::new(true, 8, "abc#nextChar");
    let parser_instance = ParserI::new(&parser, parser.pattern());

    let result = parser_instance.peek_space();
}

