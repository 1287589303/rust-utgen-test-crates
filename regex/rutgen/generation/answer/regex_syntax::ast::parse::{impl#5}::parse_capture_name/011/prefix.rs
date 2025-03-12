// Answer 0

#[test]
fn test_parse_capture_name_valid_capture_name() {
    struct TestParser {
        pos: Position,
        pattern: String,
        capture_index: u32,
    }

    impl TestParser {
        fn new(pattern: &str, capture_index: u32) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                capture_index,
            }
        }
        
        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName> {
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            let start = self.pos;
            loop {
                if self.char() == '>' {
                    break;
                }
                if !is_capture_char(self.char(), self.pos == start) {
                    return Err(self.error(self.span(), ast::ErrorKind::GroupNameInvalid));
                }
                if !self.bump() {
                    break;
                }
            }
            let end = self.pos;
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            assert_eq!(self.char(), '>');
            self.bump();
            let name = &self.pattern[start.offset..end.offset];
            if name.is_empty() {
                return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
            }
            let capname = ast::CaptureName {
                span: Span::new(start, end),
                name: name.to_string(),
                index: capture_index,
            };
            self.add_capture_name(&capname)?;
            Ok(capname)
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::GroupNameUnexpectedEof, pattern: self.pattern.clone(), span: self.span() }
        }
    }

    let mut parser = TestParser::new("<valid_name>", 1);
    let _result = parser.parse_capture_name(parser.capture_index);
}

#[test]
fn test_parse_capture_name_invalid_character() {
    struct TestParser {
        pos: Position,
        pattern: String,
        capture_index: u32,
    }

    impl TestParser {
        fn new(pattern: &str, capture_index: u32) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                capture_index,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName> {
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            let start = self.pos;
            loop {
                if self.char() == '>' {
                    break;
                }
                if !is_capture_char(self.char(), self.pos == start) {
                    return Err(self.error(self.span(), ast::ErrorKind::GroupNameInvalid));
                }
                if !self.bump() {
                    break;
                }
            }
            let end = self.pos;
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            assert_eq!(self.char(), '>');
            self.bump();
            let name = &self.pattern[start.offset..end.offset];
            if name.is_empty() {
                return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
            }
            let capname = ast::CaptureName {
                span: Span::new(start, end),
                name: name.to_string(),
                index: capture_index,
            };
            self.add_capture_name(&capname)?;
            Ok(capname)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::GroupNameUnexpectedEof, pattern: self.pattern.clone(), span: self.span() }
        }
    }

    let mut parser = TestParser::new("<invalid#name>", 1);
    let _result = parser.parse_capture_name(parser.capture_index);
}

#[test]
fn test_parse_capture_name_empty_name() {
    struct TestParser {
        pos: Position,
        pattern: String,
        capture_index: u32,
    }

    impl TestParser {
        fn new(pattern: &str, capture_index: u32) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                capture_index,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName> {
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            let start = self.pos;
            loop {
                if self.char() == '>' {
                    break;
                }
                if !is_capture_char(self.char(), self.pos == start) {
                    return Err(self.error(self.span(), ast::ErrorKind::GroupNameInvalid));
                }
                if !self.bump() {
                    break;
                }
            }
            let end = self.pos;
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            assert_eq!(self.char(), '>');
            self.bump();
            let name = &self.pattern[start.offset..end.offset];
            if name.is_empty() {
                return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
            }
            let capname = ast::CaptureName {
                span: Span::new(start, end),
                name: name.to_string(),
                index: capture_index,
            };
            self.add_capture_name(&capname)?;
            Ok(capname)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::GroupNameUnexpectedEof, pattern: self.pattern.clone(), span: self.span() }
        }
    }

    let mut parser = TestParser::new("<>", 1);
    let _result = parser.parse_capture_name(parser.capture_index);
}

#[test]
fn test_parse_capture_name_eof_before_closing() {
    struct TestParser {
        pos: Position,
        pattern: String,
        capture_index: u32,
    }

    impl TestParser {
        fn new(pattern: &str, capture_index: u32) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
                capture_index,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName> {
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            let start = self.pos;
            loop {
                if self.char() == '>' {
                    break;
                }
                if !is_capture_char(self.char(), self.pos == start) {
                    return Err(self.error(self.span(), ast::ErrorKind::GroupNameInvalid));
                }
                if !self.bump() {
                    break;
                }
            }
            let end = self.pos;
            if self.is_eof() {
                return Err(self.error(self.span(), ast::ErrorKind::GroupNameUnexpectedEof));
            }
            assert_eq!(self.char(), '>');
            self.bump();
            let name = &self.pattern[start.offset..end.offset];
            if name.is_empty() {
                return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
            }
            let capname = ast::CaptureName {
                span: Span::new(start, end),
                name: name.to_string(),
                index: capture_index,
            };
            self.add_capture_name(&capname)?;
            Ok(capname)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::GroupNameUnexpectedEof, pattern: self.pattern.clone(), span: self.span() }
        }
    }

    let mut parser = TestParser::new("<valid_name", 1);
    let _result = parser.parse_capture_name(parser.capture_index);
}

