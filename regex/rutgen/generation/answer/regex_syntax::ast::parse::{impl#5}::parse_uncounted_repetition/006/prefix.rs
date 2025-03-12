// Answer 0

#[test]
fn test_parse_uncounted_repetition_empty_concat() {
    struct MockParser {
        char: char,
        pos: Position,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = MockParser { char: 'a', pos: position };
    
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

#[test]
fn test_parse_uncounted_repetition_invalid_char() {
    struct MockParser {
        char: char,
        pos: Position,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = MockParser { char: '!', pos: position };

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);
}

