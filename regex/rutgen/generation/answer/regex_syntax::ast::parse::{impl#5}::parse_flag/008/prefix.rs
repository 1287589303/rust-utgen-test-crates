// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'i' },
        pattern: "i",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_multi_line() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'm' },
        pattern: "m",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 's' },
        pattern: "s",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_swap_greed() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'U' },
        pattern: "U",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_unicode() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'u' },
        pattern: "u",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_crlf() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'R' },
        pattern: "R",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_ignore_whitespace() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'x' },
        pattern: "x",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_unrecognized() {
    struct MockParser {
        character: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.character
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    let parser = ParserI {
        parser: MockParser { character: 'a' },
        pattern: "a",
    };
    let _ = parser.parse_flag();
}

