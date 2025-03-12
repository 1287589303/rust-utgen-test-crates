// Answer 0

#[test]
fn test_flag_duplicate() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        fn visit_flag_duplicate(&mut self, _span: Span) {}
    }

    let original_span = Span { 
        start: Position(0), 
        end: Position(1) 
    };

    let error_kind = ErrorKind::FlagDuplicate { original: original_span };

    let mut visitor = TestVisitor;
    visitor.visit_flag_duplicate(error_kind.clone());

    // Only function calls, no assertions
    let _result = format!("{}", error_kind);
}

#[test]
fn test_flag_duplicate_non_negative_offsets() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        fn visit_flag_duplicate(&mut self, _span: Span) {}
    }

    let original_span = Span { 
        start: Position(1), 
        end: Position(5) 
    };

    let error_kind = ErrorKind::FlagDuplicate { original: original_span };

    let mut visitor = TestVisitor;
    visitor.visit_flag_duplicate(error_kind.clone());

    // Only function calls, no assertions
    let _result = format!("{}", error_kind);
}

