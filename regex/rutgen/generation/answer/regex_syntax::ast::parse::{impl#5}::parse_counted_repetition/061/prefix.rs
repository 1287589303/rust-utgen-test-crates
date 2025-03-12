// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_opening_brace() {
    struct ParserMock<'s> {
        pattern: &'s str,
        pos: Position,
        char: char,
    }

    impl<'s> ParserMock<'s> {
        fn new(pattern: &'s str) -> Self {
            Self { pattern, pos: Position { offset: 0, line: 1, column: 1 }, char: 'A' }
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.to_string(), span: Span::new(self.pos, self.pos) }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }
    }

    let pattern = "a{3,5}";
    let mut concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 6, line: 1, column: 7 }), asts: vec![ast::Flags(Box::new(ast::SetFlags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 3, line: 1, column: 4 }) }))] };

    let parser = ParserMock::new(pattern);
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_with_empty_ast() {
    struct ParserMock<'s> {
        pattern: &'s str,
        pos: Position,
        char: char,
    }

    impl<'s> ParserMock<'s> {
        fn new(pattern: &'s str) -> Self {
            Self { pattern, pos: Position { offset: 0, line: 1, column: 1 }, char: 'A' }
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.to_string(), span: Span::new(self.pos, self.pos) }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }
    }

    let pattern = "a{3,5}";
    let mut concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 6, line: 1, column: 7 }), asts: vec![ast::Empty(Box::new(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })))] };

    let parser = ParserMock::new(pattern);
    let result = parser.parse_counted_repetition(concat);
}

