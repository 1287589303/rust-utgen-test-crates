// Answer 0

#[test]
fn test_parse_flags_duplicate_flags() {
    struct MockParser {
        chars: Vec<char>,
        pos: Position,
        current: usize,
    }

    impl MockParser {
        fn new(chars: &str, pos: Position) -> Self {
            Self {
                chars: chars.chars().collect(),
                pos,
                current: 0,
            }
        }

        fn char(&self) -> char {
            self.chars[self.current]
        }

        fn bump(&mut self) -> bool {
            if self.current < self.chars.len() - 1 {
                self.current += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: self.pos,
                end: self.pos,
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagDuplicate { original: self.span() },
                pattern: String::new(),
                span: self.span(),
            }
        }
        
        fn parse_flag(&self) -> Result<ast::Flag> {
            Err(self.error(self.span(), ast::ErrorKind::FlagUnrecognized)) // simulate error
        }
    }

    let parser = MockParser::new("-i-m", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_flags(); // simulate calling the function
}

#[test]
fn test_parse_flags_dangling_negation() {
    struct MockParser {
        chars: Vec<char>,
        pos: Position,
        current: usize,
    }

    impl MockParser {
        fn new(chars: &str, pos: Position) -> Self {
            Self {
                chars: chars.chars().collect(),
                pos,
                current: 0,
            }
        }

        fn char(&self) -> char {
            self.chars[self.current]
        }

        fn bump(&mut self) -> bool {
            if self.current < self.chars.len() - 1 {
                self.current += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: self.pos,
                end: self.pos,
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagDanglingNegation,
                pattern: String::new(),
                span: self.span(),
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            Err(self.error(self.span(), ast::ErrorKind::FlagUnrecognized)) // simulate error
        }
    }

    let parser = MockParser::new("-m", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_flags(); // simulate calling the function
}

#[test]
fn test_parse_flags_empty_flags() {
    struct MockParser {
        chars: Vec<char>,
        pos: Position,
        current: usize,
    }

    impl MockParser {
        fn new(chars: &str, pos: Position) -> Self {
            Self {
                chars: chars.chars().collect(),
                pos,
                current: 0,
            }
        }

        fn char(&self) -> char {
            self.chars[self.current]
        }

        fn bump(&mut self) -> bool {
            if self.current < self.chars.len() - 1 {
                self.current += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: self.pos,
                end: self.pos,
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnexpectedEof,
                pattern: String::new(),
                span: self.span(),
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            Err(self.error(self.span(), ast::ErrorKind::FlagUnrecognized)) // simulate error
        }
    }

    let parser = MockParser::new("-", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_flags(); // simulate calling the function
}

