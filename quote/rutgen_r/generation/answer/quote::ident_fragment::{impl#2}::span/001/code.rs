// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

struct IdentFragment {
    span_value: Span,
}

impl IdentFragment {
    fn span(&self) -> Span {
        self.span_value.clone()
    }
    
    fn span_option(&self) -> Option<Span> {
        Some(self.span())
    }
}

#[test]
fn test_ident_fragment_span() {
    let span_value = Span { start: 0, end: 10 };
    let ident_fragment = IdentFragment { span_value };
    let result = ident_fragment.span_option();
    assert_eq!(result, Some(Span { start: 0, end: 10 }));
}

#[test]
fn test_ident_fragment_span_boundary() {
    let span_value = Span { start: usize::MAX - 1, end: usize::MAX };
    let ident_fragment = IdentFragment { span_value };
    let result = ident_fragment.span_option();
    assert_eq!(result, Some(Span { start: usize::MAX - 1, end: usize::MAX }));
}

