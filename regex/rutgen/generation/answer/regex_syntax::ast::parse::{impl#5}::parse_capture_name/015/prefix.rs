// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        pattern: String,
        pos: Position,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                eof: false,
            }
        }
        
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('>')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::GroupNameUnexpectedEof,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut parser = MockParser::new("<valid_capture_name>");
    let capture_index = 0;
    
    // Move the parser to the location right after '<'
    parser.bump();
    
    // Call the function to test.
    let result = parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_empty_name() {
    struct MockParser {
        pattern: String,
        pos: Position,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                eof: false,
            }
        }
        
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('>')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::GroupNameEmpty,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut parser = MockParser::new("<>");
    let capture_index = 0;
    
    // Move the parser to the location right after '<'
    parser.bump();
    
    // Call the function to test.
    let result = parser.parse_capture_name(capture_index);
}

