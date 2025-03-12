// Answer 0

#[test]
fn test_len_empty_span() {
    let span = Span { start: 0, end: 0 };
    let _length = span.len();
}

#[test]
fn test_len_single_element_span() {
    let span = Span { start: 0, end: 1 };
    let _length = span.len();
}

#[test]
fn test_len_zero_length_span() {
    let span = Span { start: 1, end: 1 };
    let _length = span.len();
}

#[test]
fn test_len_non_empty_span() {
    let span = Span { start: 1, end: 2 };
    let _length = span.len();
}

#[test]
fn test_len_max_span() {
    let span = Span { start: u32::MAX as usize, end: u32::MAX as usize };
    let _length = span.len();
}

#[test]
fn test_len_max_minus_one_span() {
    let span = Span { start: (u32::MAX as usize) - 1, end: u32::MAX as usize };
    let _length = span.len();
}

