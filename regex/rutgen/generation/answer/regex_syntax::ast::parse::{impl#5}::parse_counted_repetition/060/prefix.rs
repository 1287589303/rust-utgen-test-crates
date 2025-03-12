// Answer 0

#[test]
fn test_parse_counted_repetition_bump_failure() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            'a' // self.char() != '{'
        }

        fn pos(&self) -> Position {
            Position { offset: 0, line: 1, column: 1 }
        }

        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span: Span::new(self.pos(), self.pos()) }
        }
    }

    let parser = TestParser;
    let start = parser.pos();
    let ast = ast::Ast::Flags(Box::new(ast::Flags { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })); // Populate ast with a valid Flags
    let concat = ast::Concat { span: Span::new(start, parser.pos()), asts: vec![ast] };

    let result = parser.parse_counted_repetition(concat);
}

