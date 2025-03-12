// Answer 0

#[test]
fn test_span_format_start_zero_end_one() {
    let span = Span { start: 0, end: 1 };
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", span);
}

#[test]
fn test_span_format_start_one_end_two() {
    let span = Span { start: 1, end: 2 };
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", span);
}

#[test]
fn test_span_format_start_at_max_minus_one_end_at_max() {
    let span = Span { start: usize::MAX - 1, end: usize::MAX };
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", span);
}

#[test]
fn test_span_format_start_at_max_end_above_max_should_panic() {
    #[should_panic]
    fn test_invalid_span() {
        let span = Span { start: usize::MAX, end: usize::MAX + 1 }; // Invalid case
        let mut output = Vec::new();
        let _ = write!(&mut output, "{:?}", span); // This should panic as end exceeds max
    }
    test_invalid_span();
}

#[test]
fn test_span_format_large_span() {
    let span = Span { start: 1000, end: 5000 };
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", span);
}

#[test]
fn test_span_format_reversed_span() {
    #[should_panic]
    fn test_reversed_span() {
        let span = Span { start: 2, end: 1 }; // Invalid case
        let mut output = Vec::new();
        let _ = write!(&mut output, "{:?}", span); // This should panic as start is not less than end
    }
    test_reversed_span();
}

