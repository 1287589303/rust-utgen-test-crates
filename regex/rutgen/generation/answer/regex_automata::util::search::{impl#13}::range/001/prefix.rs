// Answer 0

#[test]
fn test_range_valid_span() {
    let pattern_id = PatternID(0);
    let span = Span { start: 5, end: 10 };
    let m = Match::new(pattern_id, span);
    m.range();
}

#[test]
fn test_range_span_empty() {
    let pattern_id = PatternID(1);
    let span = Span { start: 7, end: 7 }; // empty span
    let m = Match::new(pattern_id, span);
    m.range();
}

#[test]
fn test_range_span_edge_case() {
    let pattern_id = PatternID(2);
    let span = Span { start: 0, end: 0 }; // edge case with start == end
    let m = Match::new(pattern_id, span);
    m.range();
}

#[test]
fn test_range_span_maximum() {
    let pattern_id = PatternID(3);
    let span = Span { start: 0, end: usize::MAX }; // maximum range within usize boundaries
    let m = Match::new(pattern_id, span);
    m.range();
}

