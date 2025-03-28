// Answer 0

#[test]
fn test_parse_flags_dangling_negation() {
    #[derive(Debug)]
    struct TestParser {
        pattern: String,
        pos: Position,
        ignore_whitespace: Cell<bool>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                ignore_whitespace: Cell::new(false),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or_default()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> Span {
            Span {
                start: self.pos,
                end: Position { offset: self.pos.offset + 1, line: self.pos.line, column: self.pos.column + 1 },
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            let mut flags = ast::Flags { span: self.span_char(), items: vec![] };
            let mut last_was_negation = None;
            while self.char() != ':' && self.char() != ')' {
                if self.char() == '-' {
                    last_was_negation = Some(self.span_char());
                    let item = ast::FlagsItem {
                        span: self.span_char(),
                        kind: ast::FlagsItemKind::Negation,
                    };
                    flags.add_item(item);
                } else {
                    last_was_negation = None;
                }
                if !self.bump() {
                    return Err(self.error(self.span_char(), ast::ErrorKind::FlagUnexpectedEof));
                }
            }
            if let Some(span) = last_was_negation {
                return Err(self.error(span, ast::ErrorKind::FlagDanglingNegation));
            }
            flags.span.end = self.pos;
            Ok(flags)
        }
    }

    let mut parser = TestParser::new("--)");
    let result = parser.parse_flags();
    // No assert, just calling to trigger the behavior
}

