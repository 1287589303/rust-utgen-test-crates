// Answer 0

#[derive(Debug, Clone, Copy)]
struct Span;

impl Span {
    fn new() -> Self {
        Span
    }
}

trait Spanned {
    fn __span(&self) -> Span;
}

struct ExampleStruct {
    span: Span,
}

impl Spanned for ExampleStruct {
    fn __span(&self) -> Span {
        *self.span
    }
}

#[test]
fn test_span_return_value() {
    let example = ExampleStruct {
        span: Span::new(),
    };
    let span_result = example.__span();
    assert_eq!(format!("{:?}", span_result), format!("{:?}", example.span));
}

#[test]
fn test_span_clone() {
    let example = ExampleStruct {
        span: Span::new(),
    };
    let span_result = example.__span();
    let span_clone = span_result.clone();
    assert_eq!(format!("{:?}", span_result), format!("{:?}", span_clone));
}

