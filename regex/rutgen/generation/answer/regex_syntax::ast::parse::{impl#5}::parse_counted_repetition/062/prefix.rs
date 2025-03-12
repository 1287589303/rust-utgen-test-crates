// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_start_char() {
    struct TestParser {
        char: char,
        concat: ast::Concat,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char
        }
        fn pos(&self) -> Position {
            Position { offset: 0, line: 1, column: 1 }
        }
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test"), span }
        }
        fn is_eof(&self) -> bool {
            false
        }
        fn bump_and_bump_space(&mut self) -> bool {
            true
        }
        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![Ast::empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let parser = TestParser {
        char: '}', // Invalid character
        concat,
    };

    let result = parser.parse_counted_repetition(parser.concat);
}

#[test]
fn test_parse_counted_repetition_empty_ast() {
    struct TestParser {
        char: char,
        concat: ast::Concat,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char
        }
        fn pos(&self) -> Position {
            Position { offset: 0, line: 1, column: 1 }
        }
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test"), span }
        }
        fn is_eof(&self) -> bool {
            false
        }
        fn bump_and_bump_space(&mut self) -> bool {
            true
        }
        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![Ast::empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let parser = TestParser {
        char: '{', // Invalid start character
        concat,
    };

    let result = parser.parse_counted_repetition(parser.concat);
}

