// Answer 0

#[test]
fn test_error_span_valid_range() {
    struct TestError {
        span: Span,
    }
    
    let span = Span { 
        start: Position(0), 
        end: Position(5) 
    };
    
    let error = TestError { span };
    let _ = error.span();
}

#[test]
fn test_error_span_equal_start_end() {
    struct TestError {
        span: Span,
    }
    
    let span = Span { 
        start: Position(3), 
        end: Position(3) 
    };
    
    let error = TestError { span };
    let _ = error.span();
}

#[test]
fn test_error_span_minimal_position() {
    struct TestError {
        span: Span,
    }
    
    let span = Span { 
        start: Position(u32::MIN), 
        end: Position(u32::MIN) 
    };
    
    let error = TestError { span };
    let _ = error.span();
}

#[test]
fn test_error_span_maximal_position() {
    struct TestError {
        span: Span,
    }
    
    let span = Span { 
        start: Position(u32::MAX), 
        end: Position(u32::MAX) 
    };
    
    let error = TestError { span };
    let _ = error.span();
}

#[test]
fn test_error_span_with_valid_range() {
    struct TestError {
        span: Span,
    }
    
    let span = Span { 
        start: Position(10), 
        end: Position(20) 
    };
    
    let error = TestError { span };
    let _ = error.span();
}

