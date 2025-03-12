// Answer 0

#[test]
fn test_match_creation_valid_span_equal_start_end() {
    let pattern = PatternID(0);
    let span = Span { start: 5, end: 5 };
    let m = Match::new(pattern, span);
}

#[test]
fn test_match_creation_valid_span_zero_length() {
    let pattern = PatternID(1);
    let span = Span { start: 0, end: 0 };
    let m = Match::new(pattern, span);
}

#[test]
fn test_match_creation_valid_span_non_empty() {
    let pattern = PatternID(2);
    let span = Span { start: 1, end: 10 };
    let m = Match::new(pattern, span);
}

#[test]
fn test_match_creation_valid_span_from_range() {
    let pattern = PatternID(3);
    let span = 3..3;
    let m = Match::new(pattern, span);
}

#[test]
fn test_match_creation_valid_span_from_large_range() {
    let pattern = PatternID(4);
    let span = 0..100;
    let m = Match::new(pattern, span);
}

