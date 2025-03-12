// Answer 0

#[test]
fn test_parse_hex_digits_unexpected_eof() {
    struct MockParser {
        position: Position,
        hex_digit: char,
        bump_called: bool,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                hex_digit: 'a',
                bump_called: false,
            }
        }

        fn char(&self) -> char {
            self.hex_digit
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump_called = true;
            false
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: ErrorKind::EscapeUnexpectedEof,
                pattern: "input".to_string(),
                span: Span::new(self.position, self.position),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new();
    let parser_i = ParserI { parser: &mock_parser, pattern: "\\x" };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
} 

#[test]
fn test_parse_hex_digits_unexpected_eof_2() {
    struct MockParser {
        position: Position,
        hex_digit: char,
        bump_called: bool,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                hex_digit: 'b',
                bump_called: false,
            }
        }

        fn char(&self) -> char {
            self.hex_digit
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump_called = true;
            false
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: ErrorKind::EscapeUnexpectedEof,
                pattern: "input".to_string(),
                span: Span::new(self.position, self.position),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new();
    let parser_i = ParserI { parser: &mock_parser, pattern: "\\x" };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
} 

#[test]
fn test_parse_hex_digits_unexpected_eof_3() {
    struct MockParser {
        position: Position,
        hex_digit: char,
        bump_called: bool,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                hex_digit: 'c',
                bump_called: false,
            }
        }

        fn char(&self) -> char {
            self.hex_digit
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump_called = true;
            false
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: ErrorKind::EscapeUnexpectedEof,
                pattern: "input".to_string(),
                span: Span::new(self.position, self.position),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new();
    let parser_i = ParserI { parser: &mock_parser, pattern: "\\x" };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
} 

#[test]
fn test_parse_hex_digits_unexpected_eof_4() {
    struct MockParser {
        position: Position,
        hex_digit: char,
        bump_called: bool,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                hex_digit: '1',
                bump_called: false,
            }
        }

        fn char(&self) -> char {
            self.hex_digit
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump_called = true;
            false
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: ErrorKind::EscapeUnexpectedEof,
                pattern: "input".to_string(),
                span: Span::new(self.position, self.position),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new();
    let parser_i = ParserI { parser: &mock_parser, pattern: "\\x" };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
} 

#[test]
fn test_parse_hex_digits_unexpected_eof_5() {
    struct MockParser {
        position: Position,
        hex_digit: char,
        bump_called: bool,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                hex_digit: 'f',
                bump_called: false,
            }
        }

        fn char(&self) -> char {
            self.hex_digit
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump_called = true;
            false
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            Error {
                kind: ErrorKind::EscapeUnexpectedEof,
                pattern: "input".to_string(),
                span: Span::new(self.position, self.position),
            }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mock_parser = MockParser::new();
    let parser_i = ParserI { parser: &mock_parser, pattern: "\\x" };
    let result = parser_i.parse_hex_digits(HexLiteralKind::X);
}

