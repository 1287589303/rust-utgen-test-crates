// Answer 0

#[test]
fn test_span_valid_range() {
    struct TestError {
        span: Span,
    }

    let span = Span {
        start: Position(0),
        end: Position(5),
    };

    let error = TestError { span };
    let _result = error.span();
}

#[test]
fn test_span_boundary_start_equal_end() {
    struct TestError {
        span: Span,
    }

    let span = Span {
        start: Position(5),
        end: Position(5),
    };

    let error = TestError { span };
    let _result = error.span();
}

#[test]
fn test_span_boundary_start_less_than_end() {
    struct TestError {
        span: Span,
    }

    let span = Span {
        start: Position(1),
        end: Position(2),
    };

    let error = TestError { span };
    let _result = error.span();
}

#[test]
fn test_span_invalid_range() {
    struct TestError {
        span: Span,
    }

    let span = Span {
        start: Position(5),
        end: Position(4),
    };

    let error = TestError { span };
    let _result = error.span();
}

