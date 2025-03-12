// Answer 0

#[test]
fn test_parse_set_class_with_unclosed_class_error() {
    struct MockParser {
        pub pattern: String,
        pub pos: Position,
        pub stack_class: RefCell<Vec<GroupState>>,
    }

    impl MockParser {
        pub fn new() -> Self {
            Self {
                pattern: String::from("[&&]"),
                pos: Position::default(),
                stack_class: RefCell::new(vec![]),
            }
        }

        fn char(&self) -> char {
            '['
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn peek(&self) -> Option<char> {
            Some('&')
        }

        fn bump_if(&self, _: &str) -> bool {
            true
        }

        fn bump_space(&self) {}

        fn span(&self) -> Span {
            Span {
                start: self.pos,
                end: self.pos,
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.clone(),
                span: self.span(),
            }
        }
    }

    let parser = MockParser::new();
    parser.parse_set_class().expect_err("Expected an unclosed class error");
}

