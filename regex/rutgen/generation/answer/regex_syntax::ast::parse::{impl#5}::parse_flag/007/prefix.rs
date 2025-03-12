// Answer 0

#[test]
fn test_parse_flag_multiline() {
    struct MockParser<'s> {
        pattern: &'s str,
        pos: usize,
    }

    impl<'s> MockParser<'s> {
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn error(&self, _: Position, _: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: self.pattern.to_string(),
                span: ast::Span { start: self.pos as Position, end: self.pos as Position },
            }
        }

        fn span_char(&self) -> Position {
            self.pos as Position
        }
    }

    let parser = MockParser { pattern: "m", pos: 0 };
    let result = parser.parse_flag();
}

