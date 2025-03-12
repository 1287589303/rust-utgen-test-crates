// Answer 0

#[test]
fn test_parse_counted_repetition_eof_error() {
    struct MockParser {
        pos: Position,
        char: char,
        eof: bool,
    }

    impl MockParser {
        fn new(char: char, eof: bool) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                char,
                eof,
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn char(&self) -> char {
            self.char
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 },
                        Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Literal(Box::new(ast::Literal {}))],
    };

    let parser = MockParser::new('a', true);
    let result = parser.parse_counted_repetition(concat);

    // The return value is expected to be an error of kind RepetitionCountUnclosed.
}

