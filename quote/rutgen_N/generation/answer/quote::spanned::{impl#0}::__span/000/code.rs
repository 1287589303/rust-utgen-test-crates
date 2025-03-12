// Answer 0

#[derive(Debug, Copy, Clone)]
struct Span;

impl Span {
    fn new() -> Self {
        Span
    }
}

trait Spanned {
    fn __span(&self) -> Span;
}

struct ExampleStruct;

impl Spanned for ExampleStruct {
    fn __span(&self) -> Span {
        *self.__span()
    }
}

#[test]
fn test_span_returns_correct_value() {
    let example = ExampleStruct;
    let result = example.__span();
    let expected = Span::new();
    assert_eq!(format!("{:?}", result), format!("{:?}", expected));
}

