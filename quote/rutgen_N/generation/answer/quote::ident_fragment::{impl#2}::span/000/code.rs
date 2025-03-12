// Answer 0

#[derive(Debug)]
struct Span {
    // Example fields; adapt as necessary for the actual Span structure
    start: usize,
    end: usize,
}

struct IdentFragment {
    // Assuming there is a field for span
    span: Span,
}

impl IdentFragment {
    fn span(&self) -> Span {
        self.span.clone() // Return a copy of the span
    }
    
    fn new(start: usize, end: usize) -> Self {
        Self {
            span: Span { start, end },
        }
    }
}

#[test]
fn test_span() {
    let ident_fragment = IdentFragment::new(5, 10);
    let span = ident_fragment.span().unwrap();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 10);
}

#[test]
fn test_span_empty() {
    let ident_fragment = IdentFragment::new(0, 0);
    let span = ident_fragment.span().unwrap();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

