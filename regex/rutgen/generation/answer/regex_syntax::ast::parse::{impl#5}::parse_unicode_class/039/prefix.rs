// Answer 0

#[test]
fn test_parse_unicode_class_one_letter() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
        negated: bool,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
                negated: false,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn parse_unicode_class(&mut self) -> Result<ast::ClassUnicode, ast::Error> {
            assert!(self.char() == 'p' || self.char() == 'P');
            let negated = self.char() == 'P';
            if !self.bump_and_bump_space() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::EscapeUnexpectedEof,
                    pattern: String::new(),
                    span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
                });
            }
            let start = self.pos;
            let c = self.char();
            if c == '\\' {
                return Err(ast::Error {
                    kind: ast::ErrorKind::UnicodeClassInvalid,
                    pattern: String::new(),
                    span: Span::new(Position { offset: start, line: 1, column: 1 }, Position { offset: start, line: 1, column: 1 }),
                });
            }
            self.bump_and_bump_space();
            let kind = ast::ClassUnicodeKind::OneLetter(c);
            Ok(ast::ClassUnicode {
                span: Span::new(Position { offset: start, line: 1, column: 1 }, Position { offset: self.pos, line: 1, column: 1 }),
                negated,
                kind,
            })
        }
    }

    let mut parser = MockParser::new("pZ");
    let result = parser.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_negated_one_letter() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
        negated: bool,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
                negated: true,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn parse_unicode_class(&mut self) -> Result<ast::ClassUnicode, ast::Error> {
            assert!(self.char() == 'p' || self.char() == 'P');
            let negated = self.char() == 'P';
            if !self.bump_and_bump_space() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::EscapeUnexpectedEof,
                    pattern: String::new(),
                    span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
                });
            }
            let start = self.pos;
            let c = self.char();
            if c == '\\' {
                return Err(ast::Error {
                    kind: ast::ErrorKind::UnicodeClassInvalid,
                    pattern: String::new(),
                    span: Span::new(Position { offset: start, line: 1, column: 1 }, Position { offset: start, line: 1, column: 1 }),
                });
            }
            self.bump_and_bump_space();
            let kind = ast::ClassUnicodeKind::OneLetter(c);
            Ok(ast::ClassUnicode {
                span: Span::new(Position { offset: start, line: 1, column: 1 }, Position { offset: self.pos, line: 1, column: 1 }),
                negated,
                kind,
            })
        }
    }

    let mut parser = MockParser::new("PZ");
    let result = parser.parse_unicode_class();
}

