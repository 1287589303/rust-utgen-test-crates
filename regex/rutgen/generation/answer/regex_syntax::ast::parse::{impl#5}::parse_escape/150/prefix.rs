// Answer 0

#[test]
fn test_parse_escape_with_digit() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\D"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_perl_class() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\W"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_unicode() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\p"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_hex() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\x"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
} 

#[test]
fn test_parse_escape_with_unrecognized_char() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\?"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
} 

#[test]
fn test_parse_escape_with_assertion_start() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\A"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_assertion_end() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\z"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_word_boundary() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\b"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_with_word_boundary_end() {
    struct ParserMock {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl ParserMock {
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn char(&self) -> char {
            '\\'
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = ParserMock {
        pattern: String::from("foo\\B"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let _ = parser.parse_escape();
}

