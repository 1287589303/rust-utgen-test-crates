// Answer 0

#[test]
fn test_is_empty_with_empty_span() {
    let pattern_id = PatternID(0);
    let span = Span { start: 0, end: 0 };
    let m = Match::new(pattern_id, span);
    m.is_empty();
}

#[test]
fn test_is_empty_with_non_empty_span() {
    let pattern_id = PatternID(1);
    let span = Span { start: 1, end: 5 };
    let m = Match::new(pattern_id, span);
    m.is_empty();
}

#[test]
fn test_is_empty_with_smallest_non_empty_span() {
    let pattern_id = PatternID(2);
    let span = Span { start: 5, end: 6 };
    let m = Match::new(pattern_id, span);
    m.is_empty();
}

#[test]
fn test_is_empty_with_large_span() {
    let pattern_id = PatternID(3);
    let span = Span { start: 0, end: std::usize::MAX };
    let m = Match::new(pattern_id, span);
    m.is_empty();
}

#[test]
fn test_is_empty_with_start_equals_end() {
    let pattern_id = PatternID(4);
    let span = Span { start: std::usize::MAX, end: std::usize::MAX };
    let m = Match::new(pattern_id, span);
    m.is_empty();
}

