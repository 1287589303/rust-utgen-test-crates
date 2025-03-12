// Answer 0

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    let original_span = Span {
        start: Position::new(0),
        end: Position::new(10),
    };
    
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: original_span },
        pattern: String::from("(?P<name>value)"),
        span: Span {
            start: Position::new(0),
            end: Position::new(20),
        },
    };

    let _result = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_group_name_duplicate_alternate() {
    let original_span = Span {
        start: Position::new(5),
        end: Position::new(15),
    };
    
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: original_span },
        pattern: String::from("(?P<other_name>value)"),
        span: Span {
            start: Position::new(5),
            end: Position::new(25),
        },
    };

    let _result = error.auxiliary_span();
}

