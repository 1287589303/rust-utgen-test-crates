// Answer 0

#[test]
fn test_add_capture_name_unique() {
    struct TestParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    let mut parser = TestParser {
        capture_names: RefCell::new(vec![]),
    };
    
    let cap = CaptureName {
        span: Span { start: Position::default(), end: Position::default() },
        name: String::from("unique_name"),
        index: 0,
    };
    
    let result = ParserI::new(&parser, "pattern").add_capture_name(&cap);
}

#[test]
fn test_add_capture_name_second_unique() {
    struct TestParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    let mut capture_names = RefCell::new(vec![
        CaptureName {
            span: Span { start: Position::default(), end: Position::default() },
            name: String::from("existing_name"),
            index: 1,
        },
    ]);
    
    let parser = TestParser {
        capture_names,
    };
    
    let cap = CaptureName {
        span: Span { start: Position::default(), end: Position::default() },
        name: String::from("another_unique_name"),
        index: 2,
    };
    
    let result = ParserI::new(&parser, "pattern").add_capture_name(&cap);
}

#[test]
fn test_add_capture_name_empty_name() {
    struct TestParser {
        capture_names: RefCell<Vec<CaptureName>>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }
    
    let parser = TestParser {
        capture_names: RefCell::new(vec![]),
    };
    
    let cap = CaptureName {
        span: Span { start: Position::default(), end: Position::default() },
        name: String::from(""),
        index: 3,
    };
    
    let result = ParserI::new(&parser, "pattern").add_capture_name(&cap);
}

