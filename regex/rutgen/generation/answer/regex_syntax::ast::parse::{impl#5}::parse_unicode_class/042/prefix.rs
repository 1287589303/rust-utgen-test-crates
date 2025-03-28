// Answer 0

#[test]
fn test_parse_unicode_class_success() {
    struct TestParser {
        pos: Position,
        chars: Vec<char>,
        current_index: usize,
        scratch: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                chars: pattern.chars().collect(),
                current_index: 0,
                scratch: String::new(),
            }
        }

        fn char(&self) -> char {
            *self.chars.get(self.current_index).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.current_index < self.chars.len() {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::EscapeUnexpectedEof, pattern: String::new(), span: self.span() }
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.chars.len()
        }

        fn bump(&mut self) {
            self.current_index += 1;
        }
    }

    let mut parser = TestParser::new(r"\p{Greek}");
    parser.current_index = 0; // Set to point at 'p'
    parser.bump(); // Move past 'p'
    parser.bump_and_bump_space(); // Simulate whitespace bump, returns true.
    parser.bump(); // Move to '{'

    if parser.bump_and_bump_space() {
        parser.bump(); // simulating move past '{'
    }
    
    // Call the function under test
    let _result = parser.parse_unicode_class(); // Should return Err due to EOF
}

#[test]
#[should_panic]
fn test_parse_unicode_class_fail() {
    struct TestParser {
        pos: Position,
        chars: Vec<char>,
        current_index: usize,
        scratch: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                chars: pattern.chars().collect(),
                current_index: 0,
                scratch: String::new(),
            }
        }

        fn char(&self) -> char {
            *self.chars.get(self.current_index).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            false // Simulate failure condition
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::EscapeUnexpectedEof, pattern: String::new(), span: self.span() }
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.chars.len()
        }

        fn bump(&mut self) {
            self.current_index += 1;
        }
    }

    let mut parser = TestParser::new(r"\p");
    parser.current_index = 0; // Set to point at 'p'

    // Call the function under test
    let _result = parser.parse_unicode_class(); // Should return Err due to bump_and_bump_space returning false
}

