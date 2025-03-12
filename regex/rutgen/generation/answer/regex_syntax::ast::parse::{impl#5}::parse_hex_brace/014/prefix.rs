// Answer 0

#[test]
fn test_parse_hex_brace_valid_input() {
    struct MockParser {
        hex_input: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '}' // Simulating end of the hex input.
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexInvalid, // Simulated error for testing purpose
                pattern: "error".to_string(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let parser = MockParser {
        hex_input: "1A3F".to_string(),
        position: Position { offset: 4, line: 1, column: 5 }, // Position after the char '}'.
        eof: false,
    };

    let kind = ast::HexLiteralKind::X; // Example kind

    let result = parser.parse_hex_brace(kind);
}

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct MockParser {
        hex_input: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '}' // Simulating end of the hex input.
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexEmpty,
                pattern: "error".to_string(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let parser = MockParser {
        hex_input: "".to_string(), // Empty hex input to trigger EscapeHexEmpty error.
        position: Position { offset: 1, line: 1, column: 2 }, // Position after the char '}'.
        eof: false,
    };

    let kind = ast::HexLiteralKind::X; // Example kind

    let result = parser.parse_hex_brace(kind);
}

#[test]
fn test_parse_hex_brace_invalid_hex() {
    struct MockParser {
        hex_input: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            '}' // Simulating end of the hex input.
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexInvalid,
                pattern: "error".to_string(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let parser = MockParser {
        hex_input: "XYZ".to_string(), // Invalid hex input to trigger EscapeHexInvalid error.
        position: Position { offset: 3, line: 1, column: 4 }, // Position after the char '}'.
        eof: false,
    };

    let kind = ast::HexLiteralKind::X; // Example kind

    let result = parser.parse_hex_brace(kind);
}

