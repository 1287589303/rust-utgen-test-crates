// Answer 0

#[test]
fn test_add_capture_name_duplicate() {
    struct MockParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            // Return a mocked Parser instance
            unimplemented!()
        }
    }

    let existing_capture = CaptureName {
        span: Span { start: 0, end: 1 },
        name: "duplicate".to_string(),
        index: 1,
    };

    let mut parser = MockParser {
        capture_names: RefCell::new(vec![existing_capture.clone()]),
    };

    let parser_i = ParserI::new(&parser, "test_pattern");
    
    let new_capture = CaptureName {
        span: Span { start: 2, end: 3 },
        name: "duplicate".to_string(), // Same name as existing
        index: 2,
    };

    let _result = parser_i.add_capture_name(&new_capture);
}

#[test]
fn test_add_capture_name_with_empty_name() {
    struct MockParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            // Return a mocked Parser instance
            unimplemented!()
        }
    }

    let existing_capture = CaptureName {
        span: Span { start: 0, end: 1 },
        name: "".to_string(), // An empty name is considered invalid
        index: 1,
    };

    let mut parser = MockParser {
        capture_names: RefCell::new(vec![existing_capture.clone()]),
    };

    let parser_i = ParserI::new(&parser, "test_pattern");
    
    let empty_name_capture = CaptureName {
        span: Span { start: 2, end: 3 },
        name: "".to_string(), // Empty name again
        index: 2,
    };

    let _result = parser_i.add_capture_name(&empty_name_capture);
}

