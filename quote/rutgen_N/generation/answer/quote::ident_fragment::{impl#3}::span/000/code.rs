// Answer 0

#[derive(Debug)]
struct SampleType;

impl SampleType {
    fn span(&self) -> Option<Span> {
        Some(Span)
    }
}

#[derive(Debug)]
struct Span;

#[test]
fn test_span_returns_some() {
    let sample = SampleType;
    assert!(sample.span().is_some());
}

#[test]
fn test_span_returns_specific_value() {
    let sample = SampleType;
    assert_eq!(sample.span(), Some(Span));
}

