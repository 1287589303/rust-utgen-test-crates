// Answer 0

#[test]
fn test_parse_perl_class_space() {
    struct MockParser;

    impl MockParser {
        fn char(&self) -> char {
            's'
        }

        fn span_char(&self) -> Span {
            Span { start: Position::default(), end: Position::default() }
        }
        
        fn bump(&self) {
            // Mock bump functionality
        }
    }

    let parser = MockParser;
    let result = parser.parse_perl_class();
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct MockParser;

    impl MockParser {
        fn char(&self) -> char {
            'S'
        }

        fn span_char(&self) -> Span {
            Span { start: Position::default(), end: Position::default() }
        }
        
        fn bump(&self) {
            // Mock bump functionality
        }
    }

    let parser = MockParser;
    let result = parser.parse_perl_class();
}

