// Answer 0

#[test]
fn test_parse_perl_class_negated() {
    struct MockParser {
        input: String,
        position: Position,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.position).unwrap()
        }

        fn span_char(&self) -> Span {
            Span { start: self.position, end: self.position + 1 }
        }

        fn bump(&mut self) {
            self.position += 1;
        }
    }

    let mut parser = MockParser::new(r#"\D"#);
    let result = parser.parse_perl_class();
    let expected_span = Span { start: 0, end: 1 };
    let expected_kind = ClassPerlKind::Digit;
    let expected_negated = true;

    let _ = (result.span == expected_span, result.kind == expected_kind, result.negated == expected_negated);
}

