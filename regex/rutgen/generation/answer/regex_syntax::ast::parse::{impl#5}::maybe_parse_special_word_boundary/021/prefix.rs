// Answer 0

#[test]
fn test_maybe_parse_special_word_boundary_unrecognized() {
    struct MockParser {
        input: String,
        pos: Position,
        scratch: RefCell<String>,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.column += 1;
            self.pos.offset < self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos.offset += 1;
                self.pos.column += 1;
            }
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.input.clone(), span }
        }
    }

    let mut parser = MockParser::new("{invalid}");

    let wb_start = Position { offset: 0, line: 1, column: 1 };
    parser.bump_and_bump_space(); // True
    let start_contents = parser.pos(); // Save position after bump

    // Simulate is_valid_char(self.char()) returning false
    parser.scratch.borrow_mut().extend("invalid".chars());

    let result = parser.maybe_parse_special_word_boundary(wb_start);
    let span = Span::new(start_contents, parser.pos());

    assert_eq!(result, Err(parser.error(span, ast::ErrorKind::SpecialWordBoundaryUnrecognized)));
}

