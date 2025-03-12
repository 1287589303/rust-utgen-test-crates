// Answer 0

#[test]
fn test_parse_escape_octal_not_supported() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "\\0";
    
    struct TestParser {
        octal: bool,
        position: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                position: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }

        fn bump(&self) -> bool {
            self.position.set(Position { offset: 1, line: 1, column: 2 });
            true
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.get().offset).unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn pos(&self) -> Position {
            self.position.get()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }
    }

    let parser = TestParser::new(false, pattern);
    let parser_i = ParserI { parser, pattern: pattern };

    let _result = parser_i.parse_escape();
}

