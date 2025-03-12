// Answer 0

#[test]
fn test_parse_capture_name_eof() {
    struct MockParser {
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // Indicative of EOF
            } else {
                self.pattern[self.pos.offset..].chars().next().unwrap()
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<invalid>",
    };

    let result = parser.parse_capture_name(0);
}

#[test]
fn test_parse_capture_name_empty() {
    struct MockParser {
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // Indicative of EOF
            } else {
                self.pattern[self.pos.offset..].chars().next().unwrap()
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<>",
    };

    let result = parser.parse_capture_name(1);
}

#[test]
fn test_parse_capture_name_invalid_chars() {
    struct MockParser {
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // Indicative of EOF
            } else {
                self.pattern[self.pos.offset..].chars().next().unwrap()
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<invalid#name>",
    };

    let result = parser.parse_capture_name(2);
}

#[test]
fn test_parse_capture_name_valid_capture() {
    struct MockParser {
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0' // Indicative of EOF
            } else {
                self.pattern[self.pos.offset..].chars().next().unwrap()
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<valid_name>",
    };

    let result = parser.parse_capture_name(3);
}

