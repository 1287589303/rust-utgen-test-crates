// Answer 0

#[test]
fn test_parse_set_class_open_with_negation_and_dash() {
    struct TestParser {
        pos: Position,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser { pos: Position { offset: 0, line: 1, column: 1 }, pattern: pattern.to_string() }
        }
        
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<ast::ClassBracketed> {
            Err(ast::Error { kind, pattern: self.pattern.clone(), span })
        }
    }
    
    let mut parser = TestParser::new("[^-]");
    let (set, union) = parser.parse_set_class_open().unwrap();
    // Results would be processed here if needed.
}

