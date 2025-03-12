// Answer 0

#[test]
fn test_eq_with_valid_range() {
    let range: Range<usize> = 5..10; // self.start = 5, self.end = 10
    let span = Span { start: 5, end: 10 }; // span.start = 5, span.end = 10
    let result = range.eq(&span);
}

#[test]
fn test_eq_with_same_start_different_end() {
    let range: Range<usize> = 8..12; // self.start = 8, self.end = 12
    let span = Span { start: 8, end: 12 }; // span.start = 8, span.end = 12
    let result = range.eq(&span);
}

#[test]
fn test_eq_with_boundary_conditions() {
    let range: Range<usize> = 0..1; // self.start = 0, self.end = 1
    let span = Span { start: 0, end: 1 }; // span.start = 0, span.end = 1
    let result = range.eq(&span);
}

#[should_panic]
fn test_eq_with_zero_length_range() {
    let range: Range<usize> = 2..2; // self.start = 2, self.end = 2
    let span = Span { start: 2, end: 2 }; // span.start = 2, span.end = 2
    let result = range.eq(&span);
}

#[test]
fn test_eq_with_large_values() {
    let range: Range<usize> = usize::MAX - 1..usize::MAX; // self.start = usize::MAX - 1, self.end = usize::MAX
    let span = Span { start: usize::MAX - 1, end: usize::MAX }; // span.start = usize::MAX - 1, span.end = usize::MAX
    let result = range.eq(&span);
}

